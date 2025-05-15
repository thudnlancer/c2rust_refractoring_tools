use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;
use std::mem;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

type size_t = c_ulong;

#[derive(Debug, Clone, Copy)]
pub enum VariableOrigin {
    Default = 0,
    Env = 1,
    File = 2,
    EnvOverride = 3,
    Command = 4,
    Override = 5,
    Automatic = 6,
    Invalid = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum VariableFlavor {
    Bogus = 0,
    Simple = 1,
    Recursive = 2,
    Expand = 3,
    Append = 4,
    Conditional = 5,
    Shell = 6,
    AppendValue = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum VariableExport {
    Default = 0,
    Export = 1,
    NoExport = 2,
    IfSet = 3,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub origin: VariableOrigin,
    pub flavor: VariableFlavor,
    pub export: VariableExport,
    pub recursive: bool,
    pub append: bool,
    pub conditional: bool,
    pub per_target: bool,
    pub special: bool,
    pub exportable: bool,
    pub private_var: bool,
}

impl Variable {
    pub fn new(
        name: &str,
        value: &str,
        origin: VariableOrigin,
        flavor: VariableFlavor,
        export: VariableExport,
    ) -> Self {
        Variable {
            name: name.to_string(),
            value: value.to_string(),
            origin,
            flavor,
            export,
            recursive: false,
            append: false,
            conditional: false,
            per_target: false,
            special: false,
            exportable: true,
            private_var: false,
        }
    }
}

pub struct VariableSet {
    variables: HashMap<String, Variable>,
}

impl VariableSet {
    pub fn new() -> Self {
        VariableSet {
            variables: HashMap::new(),
        }
    }

    pub fn define_variable(
        &mut self,
        name: &str,
        value: &str,
        origin: VariableOrigin,
        recursive: bool,
    ) -> &Variable {
        let var = Variable::new(
            name,
            value,
            origin,
            if recursive {
                VariableFlavor::Recursive
            } else {
                VariableFlavor::Simple
            },
            VariableExport::Default,
        );
        self.variables.insert(name.to_string(), var);
        self.variables.get(name).unwrap()
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }

    pub fn undefine_variable(&mut self, name: &str, origin: VariableOrigin) {
        if let Some(var) = self.variables.get(name) {
            if var.origin as u32 <= origin as u32 {
                self.variables.remove(name);
            }
        }
    }
}

pub struct VariableSetList {
    pub set: VariableSet,
    pub next: Option<Box<VariableSetList>>,
    pub next_is_parent: bool,
}

impl VariableSetList {
    pub fn new(set: VariableSet) -> Self {
        VariableSetList {
            set,
            next: None,
            next_is_parent: false,
        }
    }
}

pub struct PatternVar {
    pub target: String,
    pub suffix: String,
    pub variable: Variable,
    pub next: Option<Box<PatternVar>>,
}

pub struct VariableContext {
    pub global_variable_set: VariableSet,
    pub current_variable_set_list: VariableSetList,
    pub pattern_vars: Option<Box<PatternVar>>,
    pub variable_changenum: u64,
    pub env_recursion: u64,
}

impl VariableContext {
    pub fn new() -> Self {
        VariableContext {
            global_variable_set: VariableSet::new(),
            current_variable_set_list: VariableSetList::new(VariableSet::new()),
            pattern_vars: None,
            variable_changenum: 0,
            env_recursion: 0,
        }
    }

    pub fn define_variable(
        &mut self,
        name: &str,
        value: &str,
        origin: VariableOrigin,
        recursive: bool,
    ) -> &Variable {
        self.global_variable_set
            .define_variable(name, value, origin, recursive)
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&Variable> {
        let mut set_list = &self.current_variable_set_list;
        loop {
            if let Some(var) = set_list.set.lookup_variable(name) {
                return Some(var);
            }
            match &set_list.next {
                Some(next) => set_list = next,
                None => break,
            }
        }
        None
    }

    pub fn create_pattern_var(&mut self, target: &str, suffix: &str) -> &PatternVar {
        let pattern_var = PatternVar {
            target: target.to_string(),
            suffix: suffix.to_string(),
            variable: Variable::new(
                "",
                "",
                VariableOrigin::Default,
                VariableFlavor::Simple,
                VariableExport::Default,
            ),
            next: None,
        };
        let boxed = Box::new(pattern_var);
        let raw_ptr = Box::into_raw(boxed);
        unsafe {
            (*raw_ptr).next = self.pattern_vars.take();
            self.pattern_vars = Some(Box::from_raw(raw_ptr));
            &*raw_ptr
        }
    }
}

// Helper functions
pub fn variable_hash_1(key: &Variable) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.name.hash(&mut hasher);
    hasher.finish()
}

pub fn variable_hash_cmp(x: &Variable, y: &Variable) -> bool {
    x.name == y.name
}

// Main initialization
pub fn init_global_variable_set() -> VariableContext {
    let mut context = VariableContext::new();
    
    // Define automatic variables
    context.define_variable(
        "MAKELEVEL",
        "0",
        VariableOrigin::Env,
        false,
    );
    
    context.define_variable(
        "SHELL",
        "/bin/sh",
        VariableOrigin::Default,
        false,
    );
    
    context
}