use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModeChangeOp {
    Equals,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModeChangeFlag {
    Ordinary,
    XIfAnyX,
    CopyExisting,
    Done,
}

#[derive(Debug, Clone, Copy)]
pub struct ModeChange {
    pub op: ModeChangeOp,
    pub flag: ModeChangeFlag,
    pub affected: u32,
    pub value: u32,
    pub mentioned: u32,
}

impl ModeChange {
    pub fn new(op: ModeChangeOp, flag: ModeChangeFlag, affected: u32, value: u32, mentioned: u32) -> Self {
        Self { op, flag, affected, value, mentioned }
    }
}

const ALL_PERMS: u32 = 0o7777;
const USER_PERMS: u32 = 0o4700;
const GROUP_PERMS: u32 = 0o2070;
const OTHER_PERMS: u32 = 0o1007;
const ALL_READ: u32 = 0o444;
const ALL_WRITE: u32 = 0o222;
const ALL_EXEC: u32 = 0o111;
const SETUID_SETGID: u32 = 0o6000;
const STICKY: u32 = 0o1000;

fn octal_to_mode(octal: u32) -> u32 {
    if octal > 0o7777 {
        return 0;
    }
    octal
}

fn make_node_op_equals(new_mode: u32, mentioned: u32) -> Vec<ModeChange> {
    vec![
        ModeChange::new(
            ModeChangeOp::Equals,
            ModeChangeFlag::Ordinary,
            ALL_PERMS,
            new_mode,
            mentioned,
        ),
        ModeChange::new(
            ModeChangeOp::Equals,
            ModeChangeFlag::Done,
            0,
            0,
            0,
        ),
    ]
}

pub fn mode_compile(mode_string: &str) -> Option<Vec<ModeChange>> {
    let mut mc = Vec::new();
    let mut chars = mode_string.chars().peekable();

    if mode_string.chars().all(|c| c.is_digit(8)) {
        let octal_mode = u32::from_str_radix(mode_string, 8).ok()?;
        if octal_mode > 0o7777 {
            return None;
        }
        let mode = octal_to_mode(octal_mode);
        let mentioned = if mode_string.len() < 5 {
            mode & (SETUID_SETGID | STICKY | ALL_PERMS)
        } else {
            SETUID_SETGID | STICKY | ALL_PERMS
        };
        return Some(make_node_op_equals(mode, mentioned));
    }

    while let Some(c) = chars.next() {
        let mut affected = 0;
        loop {
            match c {
                'u' => affected |= USER_PERMS,
                'g' => affected |= GROUP_PERMS,
                'o' => affected |= OTHER_PERMS,
                'a' => affected |= ALL_PERMS,
                '=' | '+' | '-' => break,
                _ => return None,
            }
        }

        loop {
            let op = match c {
                '=' => ModeChangeOp::Equals,
                '+' => ModeChangeOp::Plus,
                '-' => ModeChangeOp::Minus,
                _ => return None,
            };

            let mut value = 0;
            let mut mentioned = 0;
            let mut flag = ModeChangeFlag::CopyExisting;

            match chars.peek() {
                Some('0'..='7') => {
                    let mut octal_str = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_digit(8) {
                            octal_str.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let octal_mode = u32::from_str_radix(&octal_str, 8).ok()?;
                    if octal_mode > 0o7777 {
                        return None;
                    }
                    if affected != 0 && chars.peek().is_some() && chars.peek() != Some(&',') {
                        return None;
                    }
                    mentioned = ALL_PERMS;
                    affected = mentioned;
                    value = octal_to_mode(octal_mode);
                    flag = ModeChangeFlag::Ordinary;
                }
                Some('u') => {
                    value = ALL_READ | ALL_WRITE | ALL_EXEC;
                    chars.next();
                }
                Some('g') => {
                    value = (ALL_READ | ALL_WRITE | ALL_EXEC) >> 3;
                    chars.next();
                }
                Some('o') => {
                    value = (ALL_READ | ALL_WRITE | ALL_EXEC) >> 6;
                    chars.next();
                }
                _ => {
                    flag = ModeChangeFlag::Ordinary;
                    while let Some(&c) = chars.peek() {
                        match c {
                            'r' => value |= ALL_READ,
                            'w' => value |= ALL_WRITE,
                            'x' => value |= ALL_EXEC,
                            'X' => flag = ModeChangeFlag::XIfAnyX,
                            's' => value |= SETUID_SETGID,
                            't' => value |= STICKY,
                            _ => break,
                        }
                        chars.next();
                    }
                }
            }

            mentioned = if mentioned != 0 {
                mentioned
            } else if affected != 0 {
                affected & value
            } else {
                value
            };

            mc.push(ModeChange::new(op, flag, affected, value, mentioned));

            if !matches!(chars.peek(), Some('=' | '+' | '-')) {
                break;
            }
        }

        if chars.peek() != Some(&',') {
            break;
        }
        chars.next();
    }

    if chars.next().is_none() {
        mc.push(ModeChange::new(
            ModeChangeOp::Equals,
            ModeChangeFlag::Done,
            0,
            0,
            0,
        ));
        Some(mc)
    } else {
        None
    }
}

pub fn mode_create_from_ref(ref_file: &Path) -> Option<Vec<ModeChange>> {
    let metadata = fs::metadata(ref_file).ok()?;
    let mode = metadata.mode();
    Some(make_node_op_equals(
        mode,
        SETUID_SETGID | STICKY | ALL_PERMS,
    ))
}

pub fn mode_adjust(
    oldmode: u32,
    is_dir: bool,
    umask_value: u32,
    changes: &[ModeChange],
) -> (u32, u32) {
    let mut newmode = oldmode & ALL_PERMS;
    let mut mode_bits = 0;

    for change in changes {
        if change.flag == ModeChangeFlag::Done {
            break;
        }

        let mut affected = change.affected;
        let omit_change = if is_dir { SETUID_SETGID } else { 0 } & !change.mentioned;
        let mut value = change.value;

        match change.flag {
            ModeChangeFlag::CopyExisting => {
                value &= newmode;
                value |= if value & ALL_READ != 0 { ALL_READ } else { 0 }
                    | if value & ALL_WRITE != 0 { ALL_WRITE } else { 0 }
                    | if value & ALL_EXEC != 0 { ALL_EXEC } else { 0 };
            }
            ModeChangeFlag::XIfAnyX => {
                if newmode & ALL_EXEC != 0 || is_dir {
                    value |= ALL_EXEC;
                }
            }
            _ => {}
        }

        value &= if affected != 0 { affected } else { !umask_value } & !omit_change;

        match change.op {
            ModeChangeOp::Equals => {
                let preserved = if affected != 0 { !affected } else { 0 } | omit_change;
                mode_bits |= ALL_PERMS & !preserved;
                newmode = (newmode & preserved) | value;
            }
            ModeChangeOp::Plus => {
                mode_bits |= value;
                newmode |= value;
            }
            ModeChangeOp::Minus => {
                mode_bits |= value;
                newmode &= !value;
            }
        }
    }

    (newmode, mode_bits)
}