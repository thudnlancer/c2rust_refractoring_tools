use std::ffi::CString;

#[derive(Debug, Clone)]
pub struct TabDca {
    pub id: i32,
    pub link: Option<Box<dyn std::any::Any>>,
    pub na: i32,
    pub arg: Vec<Option<CString>>,
    pub nf: i32,
    pub name: Vec<Option<CString>>,
    pub type_: Vec<i32>,
    pub num: Vec<f64>,
    pub str_: Vec<Option<CString>>,
}

impl TabDca {
    pub fn new() -> Self {
        Self {
            id: 0,
            link: None,
            na: 0,
            arg: Vec::new(),
            nf: 0,
            name: Vec::new(),
            type_: Vec::new(),
            num: Vec::new(),
            str_: Vec::new(),
        }
    }
}

pub fn glp_db_iodbc_open(dca: &TabDca, mode: i32) -> Option<Box<dyn std::any::Any>> {
    println!("iODBC table driver not supported");
    None
}

pub fn glp_db_iodbc_read(dca: &TabDca, link: &Option<Box<dyn std::any::Any>>) -> i32 {
    0
}

pub fn glp_db_iodbc_write(dca: &TabDca, link: &Option<Box<dyn std::any::Any>>) -> i32 {
    0
}

pub fn glp_db_iodbc_close(dca: &TabDca, link: &Option<Box<dyn std::any::Any>>) -> i32 {
    0
}

pub fn glp_db_mysql_open(dca: &TabDca, mode: i32) -> Option<Box<dyn std::any::Any>> {
    println!("MySQL table driver not supported");
    None
}

pub fn glp_db_mysql_read(dca: &TabDca, link: &Option<Box<dyn std::any::Any>>) -> i32 {
    0
}

pub fn glp_db_mysql_write(dca: &TabDca, link: &Option<Box<dyn std::any::Any>>) -> i32 {
    0
}

pub fn glp_db_mysql_close(dca: &TabDca, link: &Option<Box<dyn std::any::Any>>) -> i32 {
    0
}