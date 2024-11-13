use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use syn::{visit::Visit, DeriveInput, Item, ItemStruct, ItemEnum, ItemFn, ItemUnion};

// 定义一个访问器来收集使用位置
#[derive(Serialize, Deserialize)]
struct LocationCollector {
    structs: Vec<String>,
    enums: Vec<String>,
    functions: Vec<String>,
    unions: Vec<String>
}

impl<'ast> Visit<'ast> for LocationCollector {
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.structs.push(i.ident.to_string());
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.enums.push(i.ident.to_string());
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.functions.push(i.sig.ident.to_string());
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_union(&mut self, i: &'ast ItemUnion) {
        self.unions.push(i.ident.to_string());
        syn::visit::visit_item_union(self, i);
    }
}

// 递归遍历目录中的所有.rs文件
fn process_files(dir: &Path, collector: &mut LocationCollector) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                process_files(&path, collector)?;
            } else if path.extension().map_or(false, |e| e == "rs") {
                let contents = fs::read_to_string(&path)?;
                let file = syn::parse_file(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
                //file.visit_file(&mut collector);
                collector.visit_file(&file);
                println!("Structs in {}: {:?}", path.display(), collector.structs);
                println!("Enums in {}: {:?}", path.display(), collector.enums);
                println!("Functions in {}: {:?}", path.display(), collector.functions);
                println!("Unions in {}: {:?}", path.display(), collector.unions);
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let root_path = Path::new("../c_prog/binn");
    let mut collector = LocationCollector { structs: vec![], enums: vec![], functions: vec![], unions: vec![] };
    process_files(root_path, &mut collector);

    let file = File::create("locations.json")?;
    serde_json::to_writer_pretty(file, &collector)?;

    Ok(())
}
