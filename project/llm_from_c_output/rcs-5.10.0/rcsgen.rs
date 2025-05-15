use std::ffi::CStr;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::mem;
use std::path::Path;
use std::ptr;
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

use libc::{c_char, c_int, c_void, size_t};

use crate::base::*;
use crate::b_complain::*;
use crate::b_divvy::*;
use crate::b_esds::*;
use crate::b_fb::*;
use crate::b_feph::*;
use crate::b_fro::*;
use crate::b_kwxout::*;

enum StringWork {
    Enter,
    Copy,
    Edit,
    Expand,
    EditExpand,
}

fn scan_delta_text(
    es: &mut EditStuff,
    ls: &mut Option<Box<WLink>>,
    delta: &Delta,
    func: StringWork,
    need_log: bool,
) {
    let mut next_delta;
    let from = FLOW!(from);
    let to = FLOW!(to);
    let mut log;
    let mut text;
    let mut range = Range::default();

    loop {
        next_delta = ls.as_ref().unwrap().entry;
        log = next_delta.log;
        text = next_delta.text;
        range.beg = next_delta.neck;
        range.end = text.beg;

        if need_log && delta == next_delta {
            delta.pretty_log = string_from_atat(SINGLE, log);
            delta.pretty_log = clean_log_msg(delta.pretty_log.string, delta.pretty_log.size);
        }

        if to.is_some() {
            fro_spew_partial(to.as_mut().unwrap(), from, &range);
        }

        if delta == next_delta {
            break;
        }

        if to.is_some() {
            atat_put(to.as_mut().unwrap(), text);
        }

        *ls = ls.as_mut().unwrap().next.take();
    }

    fro_move(from, range.end);

    match func {
        StringWork::Enter => enter_string(es, text),
        StringWork::Copy => copy_string(es, text),
        StringWork::Expand => {
            let c;
            let mut ctx = ExpCtx::new(FLOW!(res), to, from, true, true);

            c = get_char(from);
            if to.is_some() {
                afputc(c, to.as_mut().unwrap());
            }
            while expand_line(&mut ctx) > 1 {
                continue;
            }
            finish_exp_ctx(&mut ctx);
        }
        StringWork::Edit => edit_string(es, text, None),
        StringWork::EditExpand => edit_string(es, text, Some(delta)),
    }
}

fn build_revision(
    deltas: Option<Box<WLink>>,
    target: &Delta,
    outfile: Option<File>,
    expand_flag: bool,
) -> Option<String> {
    let mut es = make_edit_stuff();
    let mut ls = GROK!(deltas);

    if deltas.as_ref().unwrap().entry == target {
        open_fcopy(outfile.as_ref());
        scan_delta_text(
            &mut es,
            &mut ls,
            target,
            if expand_flag { StringWork::Expand } else { StringWork::Copy },
            true,
        );
    } else {
        scan_delta_text(&mut es, &mut ls, deltas.as_ref().unwrap().entry, StringWork::Enter, false);
        
        let mut deltas = deltas;
        while {
            ls = ls.unwrap().next;
            deltas = deltas.unwrap().next;
            deltas.as_ref().unwrap().next.is_some()
        } {
            scan_delta_text(&mut es, &mut ls, deltas.as_ref().unwrap().entry, StringWork::Edit, false);
        }

        if expand_flag || outfile.is_some() {
            finish_edit(&mut es, None, outfile.as_ref(), false);
        }

        scan_delta_text(
            &mut es,
            &mut ls,
            target,
            if expand_flag { StringWork::EditExpand } else { StringWork::Edit },
            true,
        );

        finish_edit(
            &mut es,
            if expand_flag { Some(target) } else { None },
            outfile.as_ref(),
            true,
        );
    }

    unmake_edit_stuff(es);
    if outfile.is_some() {
        None
    } else {
        OZclose(&mut FLOW!(res));
        Some(FLOW!(result).to_string())
    }
}

fn clean_log_msg(m: &str, s: usize) -> Cbuf {
    let mut m = m;
    let mut s = s;

    while s > 0 && (m.starts_with(' ') || m.starts_with('\t') || m.starts_with('\n')) {
        m = &m[1..];
        s -= 1;
    }

    while s > 0 && (m.ends_with(' ') || m.ends_with('\t') || m.ends_with('\n')) {
        s -= 1;
    }

    Cbuf {
        string: m,
        size: s,
    }
}

fn tty_stdin() -> bool {
    static INTERACTIVE_VALID: AtomicBool = AtomicBool::new(false);
    static INTERACTIVE: AtomicBool = AtomicBool::new(false);
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        let interactive = unsafe { libc::isatty(libc::STDIN_FILENO) } != 0;
        INTERACTIVE.store(interactive, Ordering::Relaxed);
        INTERACTIVE_VALID.store(true, Ordering::Relaxed);
    });

    INTERACTIVE.load(Ordering::Relaxed)
}

fn getc_stdin() -> i32 {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    if stdin.fill_buf().unwrap().is_empty() && tty_stdin() {
        stdin.consume(0);
    }

    match stdin.read_byte() {
        Ok(c) => c as i32,
        Err(e) => {
            if e.kind() == io::ErrorKind::UnexpectedEof && tty_stdin() {
                complain("\n");
            }
            -1
        }
    }
}

fn yes_or_no(default_answer: bool, question: &str) -> bool {
    if !BE!(quiet) && tty_stdin() {
        let ans = if default_answer { "yn" } else { "ny" };

        oflush();
        complain!("{}? [{}]({}): ", question, ans, ans.chars().next().unwrap());

        let r = getc_stdin();
        while getc_stdin() != '\n' as i32 {}

        match r as u8 as char {
            'y' | 'Y' => true,
            'n' | 'N' => false,
            _ => default_answer,
        }
    } else {
        default_answer
    }
}

fn write_desc_maybe(to: Option<&mut File>) {
    let desc = GROK!(desc);

    if let Some(to) = to {
        atat_put(to, desc);
    }
}

fn put_desc(cb: &mut Cbuf, text_flag: bool, text_file: Option<&str>) {
    let frew = FLOW!(rewr);
    let from = FLOW!(from);

    if from.is_some() && !text_flag {
        aprintf!(frew, "\n\n{}\n", TINYKS!(desc));
        write_desc_maybe(frew);
    } else {
        FLOW!(to) = None;
        aprintf!(frew, "\n\n{}\n", TINYKS!(desc));

        if text_file.is_none() {
            *cb = gets_stdin("t-", "description", "NOTE: This is NOT the log message!\n");
        } else if cb.string.is_empty() {
            let (p, s) = if text_file.unwrap().starts_with('-') {
                let p = &text_file.unwrap()[1..];
                (p.to_string(), p.len())
            } else {
                let mut txt = File::open(text_file.unwrap()).unwrap();
                let mut buf = Vec::new();
                txt.read_to_end(&mut buf).unwrap();
                (String::from_utf8(buf).unwrap(), buf.len())
            };

            *cb = clean_log_msg(&p, s);
        }

        put_string(frew, cb, true);
        newline(frew);
    }
}

fn gets_stdin(option: &str, name: &str, note: &str) -> Cbuf {
    let mut len = 0;
    let mut column = 0;
    let mut dot_in_first_column = false;
    let mut discard = false;
    let mut buf = Vec::new();

    if tty_stdin() {
        complain!(
            "enter {}, terminated with single '.' or end of file:\n{}>> ",
            name,
            note
        );
    } else if io::stdin().fill_buf().unwrap().is_empty() {
        fatal!("can't reread redirected stdin for {}; use -{}<{}>", name, option, name);
    }

    loop {
        let c = getc_stdin();
        if c == -1 {
            break;
        }

        if column == 0 {
            dot_in_first_column = c as u8 as char == '.';
        }

        if c as u8 as char == '\n' {
            if column == 1 && dot_in_first_column {
                discard = true;
                break;
            } else if tty_stdin() {
                complain!(">> ");
            }
            column = 0;
        } else {
            column += 1;
        }

        buf.push(c as u8);
    }

    let p = String::from_utf8(buf).unwrap();
    clean_log_msg(&p, p.len() - if discard { 1 } else { 0 })
}

fn format_assocs(out: &mut File, fmt: &str) {
    for ls in GROK!(symbols) {
        let d = ls.entry;

        aprintf!(out, fmt, d.meaningful, d.underlying);
    }
}

fn format_locks(out: &mut File, fmt: &str) {
    for ls in GROK!(locks) {
        let rl = ls.entry;

        aprintf!(out, fmt, rl.login, rl.delta.num);
    }
}

fn put_admin() {
    let fout = FLOW!(rewr).unwrap_or_else(|| {
        if BAD_CREAT0 {
            ORCSclose();
            File::create(makedirtemp(false)).unwrap()
        } else {
            let fo = REPO!(fd_lock);
            REPO!(fd_lock) = -1;
            File::from_raw_fd(fo)
        }
    });

    let r = REPO!(r);
    let tip = REPO!(tip);
    let defbr = if r.is_some() { GROK!(branch) } else { None };
    let kws = BE!(kws);

    aprintf!(
        fout,
        "{}\t{}{}",
        TINYKS!(head),
        tip.map_or("", |t| t.num),
        SEMI_LF
    );

    if defbr.is_some() && VERSION!(4) <= BE!(version) {
        aprintf!(fout, "{}\t{}{}", TINYKS!(branch), defbr.unwrap(), SEMI_LF);
    }

    aputs!(fout, TINYKS!(access));
    for ls in r.map_or(Vec::new(), |r| GROK!(access)) {
        aprintf!(fout, "\n\t{}", ls.entry);
    }
    aprintf!(fout, "{}", SEMI_LF);

    aprintf!(fout, "{}", TINYKS!(symbols));
    format_assocs(fout, "\n\t%s:%s");
    aprintf!(fout, "{}", SEMI_LF);

    aprintf!(fout, "{}", TINYKS!(locks));
    if r.is_some() {
        format_locks(fout, "\n\t%s:%s");
    }
    if BE!(strictly_locking) {
        aprintf!(fout, "; {}", TINYKS!(strict));
    }
    aprintf!(fout, "{}", SEMI_LF);

    if GROK!(integrity).is_some() {
        aprintf!(fout, "{}\n", TINYKS!(integrity));
        atat_put(fout, GROK!(integrity).unwrap());
        aprintf!(fout, "{}", SEMI_LF);
    }

    if REPO!(log_lead).size > 0 {
        aprintf!(fout, "{}\t", TINYKS!(comment));
        put_string(fout, &REPO!(log_lead), false);
        aprintf!(fout, "{}", SEMI_LF);
    }

    if kws != kwsub_kv {
        aprintf!(
            fout,
            "{}\t{}{}{}{}",
            TINYKS!(expand),
            SDELIM,
            kwsub_string(kws),
            SDELIM,
            SEMI_LF
        );
    }

    aprintf!(fout, "\n");
}

fn put_delta(node: &Delta, fout: &mut File) {
    if node.is_none() {
        return;
    }

    aprintf!(
        fout,
        "\n{}\n{}\t{};\t{} {};\t{} {}{}{}",
        node.num,
        TINYKS!(date),
        node.date,
        TINYKS!(author),
        node.author,
        TINYKS!(state),
        node.state.unwrap_or(""),
        SEMI_LF,
        TINYKS!(branches)
    );

    for ls in node.branches {
        let delta = ls.entry;
        aprintf!(fout, "\n\t{}", delta.num);
    }

    aprintf!(fout, "{}", SEMI_LF);
    aprintf!(
        fout,
        "{}\t{}",
        TINYKS!(next),
        node.ilk.map_or("", |i| i.num)
    );
    aprintf!(fout, "{}", SEMI_LF);

    if node.commitid.is_some() {
        aprintf!(
            fout,
            "{}\t{}{}",
            TINYKS!(commitid),
            node.commitid.unwrap(),
            SEMI_LF
        );
    }
}

fn put_tree(root: Option<&Delta>, fout: &mut File) {
    let mut root = root;
    while let Some(node) = root {
        if node.selector {
            put_delta(node, fout);
        }

        let mut ls = node.branches;
        if ls.is_none() {
            root = node.ilk;
        } else {
            put_tree(node.ilk, fout);
            while let Some(next) = ls.unwrap().next {
                put_tree(ls.unwrap().entry, fout);
                ls = next;
            }
            root = ls.unwrap().entry;
        }
    }
}

fn put_d_text(
    delta: &Delta,
    srcname: &str,
    fout: &mut File,
    diffmt: bool,
) -> bool {
    let fin = fro_open(srcname, "r", None);
    if fin.is_none() {
        syserror_errno(srcname);
        return false;
    }

    put_df_text(delta, fin.unwrap(), fout, diffmt);
    fro_close(fin.unwrap());
    true
}

fn put_sdelim(out: &mut File) {
    aputc(SDELIM as i32, out);
}

fn put_string(out: &mut File, s: &Cbuf, log: bool) {
    put_sdelim(out);

    let mut sp = s.string;
    let mut ss = s.size;

    while ss > 0 {
        let delim = memchr(sp, SDELIM as u8, ss);
        let span = delim.map_or(ss, |d| (d as usize - sp as usize) + 1);

        awrite(sp, span, out);
        if delim.is_some() {
            put_sdelim(out);
        }

        sp = unsafe { sp.add(span) };
        ss -= span;
    }

    if s.size > 0 && log {
        newline(out);
    }

    put_sdelim(out);
}

fn put_df_text(delta: &Delta, fin: &mut Fro, fout: &mut File, diffmt: bool) {
    aprintf!(
        fout,
        "\n\n{}\n{}\n",
        delta.num,
        TINYKS!(log)
    );

    put_string(fout, &delta.pretty_log, true);
    newline(fout);
    aprintf!(fout, "{}\n{}", TINYKS!(text), SDELIM as char);

    if !diffmt {
        loop {
            let c = get_char_or(fin, break);
            if c == SDELIM as i32 {
                put_sdelim(fout);
            }
            aputc(c, fout);
        }
    } else {
        let mut dc = DiffCmd::default();
        init_diff_cmd(&mut dc);

        loop {
            let ed = get_diff_cmd(fin, false, fout, &mut dc);
            if ed < 0 {
                break;
            }

            if ed != 0 {
                while dc.nlines > 0 {
                    dc.nlines -= 1;
                    loop {
                        let c = get_char_or(fin, {
                            if dc.nlines == 0 {
                                break;
                            }
                            unexpected_eof();
                        });

                        if c == SDELIM as i32 {
                            put_sdelim(fout);
                        }
                        aputc(c, fout);

                        if c == '\n' as i32 {
                            break;
                        }
                    }
                }
            }
        }
    }

    aprintf!(fout, "{}\n", SDELIM as char);
}