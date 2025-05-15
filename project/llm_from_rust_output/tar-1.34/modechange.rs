use std::ffi::CStr;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

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

#[derive(Debug)]
pub enum ModeError {
    InvalidModeString,
    StatFailed,
}

const MODE_MASK: u32 = 0o7777;
const ALL_PERMS: u32 = 0o7777;
const USER_PERMS: u32 = 0o700;
const GROUP_PERMS: u32 = 0o070;
const OTHER_PERMS: u32 = 0o007;
const SPECIAL_PERMS: u32 = 0o7000;

pub fn octal_to_mode(octal: u32) -> u32 {
    octal & MODE_MASK
}

pub fn make_node_op_equals(new_mode: u32, mentioned: u32) -> Vec<ModeChange> {
    vec![
        ModeChange {
            op: ModeChangeOp::Equals,
            flag: ModeChangeFlag::Ordinary,
            affected: ALL_PERMS,
            value: new_mode,
            mentioned,
        },
        ModeChange {
            op: ModeChangeOp::Equals,
            flag: ModeChangeFlag::Done,
            affected: 0,
            value: 0,
            mentioned: 0,
        },
    ]
}

pub fn mode_compile(mode_string: &str) -> Result<Vec<ModeChange>, ModeError> {
    if mode_string.chars().all(|c| c.is_digit(8)) {
        let octal_mode = u32::from_str_radix(mode_string, 8).map_err(|_| ModeError::InvalidModeString)?;
        if octal_mode > MODE_MASK {
            return Err(ModeError::InvalidModeString);
        }
        
        let mode = octal_to_mode(octal_mode);
        let mentioned = if mode_string.len() < 5 {
            mode & (SPECIAL_PERMS | USER_PERMS | GROUP_PERMS | OTHER_PERMS)
        } else {
            ALL_PERMS
        };
        
        return Ok(make_node_op_equals(mode, mentioned));
    }

    let mut changes = Vec::new();
    let mut chars = mode_string.chars().peekable();

    while let Some(c) = chars.next() {
        let mut affected = 0;
        
        match c {
            'u' => affected |= SPECIAL_PERMS | USER_PERMS,
            'g' => affected |= SPECIAL_PERMS | GROUP_PERMS,
            'o' => affected |= SPECIAL_PERMS | OTHER_PERMS,
            'a' => affected |= ALL_PERMS,
            '=' | '+' | '-' => {
                let op = match c {
                    '=' => ModeChangeOp::Equals,
                    '+' => ModeChangeOp::Plus,
                    '-' => ModeChangeOp::Minus,
                    _ => unreachable!(),
                };
                
                let mut value = 0;
                let mut mentioned = 0;
                let mut flag = ModeChangeFlag::CopyExisting;
                
                if let Some(&next) = chars.peek() {
                    if next.is_digit(8) {
                        let octal_str: String = chars.by_ref().take_while(|c| c.is_digit(8)).collect();
                        let octal_mode = u32::from_str_radix(&octal_str, 8)
                            .map_err(|_| ModeError::InvalidModeString)?;
                        
                        if octal_mode > MODE_MASK {
                            return Err(ModeError::InvalidModeString);
                        }
                        
                        mentioned = ALL_PERMS;
                        affected = mentioned;
                        value = octal_to_mode(octal_mode);
                        flag = ModeChangeFlag::Ordinary;
                    } else {
                        // Handle symbolic modes
                        unimplemented!("Symbolic mode parsing not fully implemented");
                    }
                }
                
                changes.push(ModeChange {
                    op,
                    flag,
                    affected,
                    value,
                    mentioned: if mentioned != 0 {
                        mentioned
                    } else if affected != 0 {
                        affected & value
                    } else {
                        value
                    },
                });
            }
            ',' => continue,
            _ => return Err(ModeError::InvalidModeString),
        }
    }

    changes.push(ModeChange {
        op: ModeChangeOp::Equals,
        flag: ModeChangeFlag::Done,
        affected: 0,
        value: 0,
        mentioned: 0,
    });

    Ok(changes)
}

pub fn mode_create_from_ref(ref_file: &Path) -> Result<Vec<ModeChange>, ModeError> {
    let metadata = std::fs::metadata(ref_file).map_err(|_| ModeError::StatFailed)?;
    let mode = metadata.mode() & MODE_MASK;
    Ok(make_node_op_equals(mode, ALL_PERMS))
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
        let omit_change = if is_dir { SPECIAL_PERMS & !change.mentioned } else { 0 };
        let mut value = change.value;

        match change.flag {
            ModeChangeFlag::CopyExisting => {
                value &= newmode;
                // Reconstruct full permission bits
                unimplemented!("CopyExisting flag handling not fully implemented");
            }
            ModeChangeFlag::XIfAnyX => {
                if (newmode & 0o111) != 0 || is_dir {
                    value |= 0o111;
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