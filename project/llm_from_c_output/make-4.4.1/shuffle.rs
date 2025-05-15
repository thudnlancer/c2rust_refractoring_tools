use std::ffi::CStr;
use std::ptr;
use std::mem;
use std::cmp;
use std::collections::HashSet;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShuffleMode {
    None,
    Random,
    Reverse,
    Identity,
}

struct Config {
    mode: ShuffleMode,
    seed: u32,
    strval: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: ShuffleMode::None,
            seed: 0,
            strval: String::new(),
        }
    }
}

static mut CONFIG: Config = Config {
    mode: ShuffleMode::None,
    seed: 0,
    strval: String::new(),
};

pub fn shuffle_get_mode() -> Option<&'static str> {
    unsafe {
        if CONFIG.strval.is_empty() {
            None
        } else {
            Some(&CONFIG.strval)
        }
    }
}

pub fn shuffle_set_mode(cmdarg: &str) -> Result<(), String> {
    let cmdarg_lower = cmdarg.to_lowercase();
    unsafe {
        match cmdarg_lower.as_str() {
            "reverse" => {
                CONFIG.mode = ShuffleMode::Reverse;
                CONFIG.strval = "reverse".to_string();
            }
            "identity" => {
                CONFIG.mode = ShuffleMode::Identity;
                CONFIG.strval = "identity".to_string();
            }
            "none" => {
                CONFIG.mode = ShuffleMode::None;
                CONFIG.strval.clear();
            }
            "random" => {
                let mut rng = StdRng::from_entropy();
                CONFIG.seed = rng.gen();
                CONFIG.mode = ShuffleMode::Random;
                CONFIG.strval = CONFIG.seed.to_string();
            }
            _ => {
                match cmdarg.parse::<u32>() {
                    Ok(seed) => {
                        CONFIG.seed = seed;
                        CONFIG.mode = ShuffleMode::Random;
                        CONFIG.strval = seed.to_string();
                    }
                    Err(_) => return Err(format!("invalid shuffle mode: '{}'", cmdarg)),
                }
            }
        }
    }
    Ok(())
}

fn random_shuffle_array<T>(a: &mut [T]) {
    let mut rng = StdRng::seed_from_u64(unsafe { CONFIG.seed as u64 });
    for i in 0..a.len() {
        let j = rng.gen_range(0..a.len());
        if i != j {
            a.swap(i, j);
        }
    }
}

fn reverse_shuffle_array<T>(a: &mut [T]) {
    for i in 0..a.len() / 2 {
        let j = a.len() - 1 - i;
        a.swap(i, j);
    }
}

fn identity_shuffle_array<T>(_a: &mut [T]) {
    // No-op
}

struct Dep {
    next: Option<Box<Dep>>,
    wait_here: bool,
    shuf: Option<Box<Dep>>,
    file: Option<Box<File>>,
}

struct File {
    deps: Option<Box<Dep>>,
    was_shuffled: bool,
}

fn shuffle_deps(deps: &mut Dep) {
    if deps.wait_here {
        return;
    }

    let mut ndeps = 0;
    let mut dep = deps;
    while let Some(next) = dep.next.as_deref_mut() {
        if next.wait_here {
            return;
        }
        ndeps += 1;
        dep = next;
    }

    if ndeps == 0 {
        return;
    }

    let mut da: Vec<&mut Dep> = Vec::with_capacity(ndeps);
    let mut dep = deps;
    while let Some(next) = dep.next.as_deref_mut() {
        da.push(dep);
        dep = next;
    }

    unsafe {
        match CONFIG.mode {
            ShuffleMode::Random => random_shuffle_array(&mut da),
            ShuffleMode::Reverse => reverse_shuffle_array(&mut da),
            ShuffleMode::Identity => identity_shuffle_array(&mut da),
            ShuffleMode::None => (),
        }
    }

    let mut prev = None;
    for dep in da.into_iter().rev() {
        dep.shuf = prev.take().map(Box::new);
        prev = Some(dep);
    }
}

fn shuffle_file_deps_recursive(f: &mut File, visited: &mut HashSet<*const File>) {
    if f.was_shuffled || visited.contains(&(f as *const _)) {
        return;
    }
    visited.insert(f as *const _);
    f.was_shuffled = true;

    if let Some(deps) = &mut f.deps {
        shuffle_deps(deps);
        let mut dep = deps;
        while let Some(next) = dep.next.as_deref_mut() {
            if let Some(file) = &mut next.file {
                shuffle_file_deps_recursive(file, visited);
            }
            dep = next;
        }
    }
}

pub fn shuffle_deps_recursive(deps: &mut Dep, not_parallel: bool) {
    unsafe {
        if CONFIG.mode == ShuffleMode::None || not_parallel {
            return;
        }

        if CONFIG.mode == ShuffleMode::Random {
            // Seed already set in shuffle_set_mode
        }

        shuffle_deps(deps);

        let mut visited = HashSet::new();
        let mut dep = deps;
        while let Some(next) = dep.next.as_deref_mut() {
            if let Some(file) = &mut next.file {
                shuffle_file_deps_recursive(file, &mut visited);
            }
            dep = next;
        }
    }
}

#[macro_export]
macro_rules! shuffle_goaldeps_recursive {
    ($g:expr, $np:expr) => {
        shuffle_deps_recursive($g, $np);
    };
}