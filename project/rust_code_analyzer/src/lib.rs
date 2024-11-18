use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::path::Path;
use syn::{visit::Visit, ItemStruct, ItemEnum, ItemFn, ItemUnion, spanned::Spanned};
use std::collections::HashMap;


// 定义访问器来收集使用位置
#[derive(Serialize, Deserialize)]
struct StructCollector {
    file_name: String,
    structs: HashMap<String, Vec<(String, usize, usize)>>
}

#[derive(Serialize, Deserialize)]
struct EnumCollector {
    file_name: String,
    enums: HashMap<String, Vec<(String, usize, usize)>>
}

#[derive(Serialize, Deserialize)]
struct FunctionCollector {
    file_name: String,
    functions: HashMap<String, Vec<(String, usize, usize)>>
}

#[derive(Serialize, Deserialize)]
struct UnionCollector {
    file_name: String,
    unions: HashMap<String, Vec<(String, usize, usize)>>
}

impl<'ast> Visit<'ast> for StructCollector {
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let struct_name = i.ident.to_string();
        let line = i.span().start().line;
        let column = i.span().start().column;

        if self.structs.contains_key(&struct_name) {
            self.structs.get_mut(&struct_name).expect("REASON").push((self.file_name.clone(), line, column));
        }
        else {
            self.structs.insert(struct_name.clone(), Vec::new());
            self.structs.get_mut(&struct_name).expect("REASON").push((self.file_name.clone(), line, column));
        }
        syn::visit::visit_item_struct(self, i);
    }
}

impl<'ast> Visit<'ast> for EnumCollector {
    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        let enum_name = i.ident.to_string();
        let line = i.span().start().line;
        let column = i.span().start().column;

        if self.enums.contains_key(&enum_name) {
            self.enums.get_mut(&enum_name).expect("REASON").push((self.file_name.clone(), line, column));
        }
        else {
            self.enums.insert(enum_name.clone(), Vec::new());
            self.enums.get_mut(&enum_name).expect("REASON").push((self.file_name.clone(), line, column));
        }
        syn::visit::visit_item_enum(self, i);
    }
}

impl<'ast> Visit<'ast> for FunctionCollector {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let func_name = i.sig.ident.to_string();
        let line = i.span().start().line;
        let column = i.span().start().column;

        if self.functions.contains_key(&func_name) {
            self.functions.get_mut(&func_name).expect("REASON").push((self.file_name.clone(), line, column));
        }
        else {
            self.functions.insert(func_name.clone(), Vec::new());
            self.functions.get_mut(&func_name).expect("REASON").push((self.file_name.clone(), line, column));
        }
        syn::visit::visit_item_fn(self, i);
    }
}

impl<'ast> Visit<'ast> for UnionCollector {
    fn visit_item_union(&mut self, i: &'ast ItemUnion) {
        let union_name = i.ident.to_string();
        let line = i.span().start().line;
        let column = i.span().start().column;

        if self.unions.contains_key(&union_name) {
            self.unions.get_mut(&union_name).expect("REASON").push((self.file_name.clone(), line, column));
        }
        else {
            self.unions.insert(union_name.clone(), Vec::new());
            self.unions.get_mut(&union_name).expect("REASON").push((self.file_name.clone(), line, column));
        }
        syn::visit::visit_item_union(self, i);
    }
}

// 递归遍历目录中的所有.rs文件
fn process_files(dir: &Path, struct_collector: &mut StructCollector, enum_collector: &mut EnumCollector, function_collector: &mut FunctionCollector, union_collector: &mut UnionCollector) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                process_files(&path, struct_collector, enum_collector, function_collector, union_collector)?;
            } else if path.extension().map_or(false, |e| e == "rs") {
                let contents = fs::read_to_string(&path)?;
                let file = syn::parse_file(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
                //file.visit_file(&mut collector);
                struct_collector.file_name = path.display().to_string();
                struct_collector.visit_file(&file);

                enum_collector.file_name = path.display().to_string();
                enum_collector.visit_file(&file);

                function_collector.file_name = path.display().to_string();
                function_collector.visit_file(&file);

                union_collector.file_name = path.display().to_string();
                union_collector.visit_file(&file);
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let full_path = Path::new("../location_rust");
    if !full_path.exists() {
        // 如果路径不存在，则创建它
        if let Err(e) = fs::create_dir_all(&full_path) {
            // 处理可能的错误
            eprintln!("无法创建文件夹 '{}': {}", full_path.display(), e);
        } else {
            println!("文件夹 '{}' 创建成功或已存在。", full_path.display());
        }
    } else {
        println!("文件夹 '{}' 已存在。", full_path.display());
    }

    let root_path = Path::new("../../c_prog");
    let mut struct_collector = StructCollector { file_name : String::new(), structs: HashMap::new() };
    let mut enum_collector = EnumCollector { file_name : String::new(), enums: HashMap::new() };
    let mut function_collector = FunctionCollector { file_name : String::new(), functions: HashMap::new() };
    let mut union_collector = UnionCollector { file_name : String::new(), unions: HashMap::new() };

    for entry in fs::read_dir(root_path)? {
        let entry = entry?;
        let path = entry.path();

        let _ = process_files(&path, &mut struct_collector, &mut enum_collector, &mut function_collector, &mut union_collector);

        let result_path = Path::new("../location_rust").join(entry.file_name());
        if !result_path.exists() {
            // 如果路径不存在，则创建它
            if let Err(e) = fs::create_dir_all(&result_path) {
                // 处理可能的错误
                eprintln!("无法创建文件夹 '{}': {}", result_path.display(), e);
            } else {
                println!("文件夹 '{}' 创建成功或已存在。", result_path.display());
            }
        } else {
            println!("文件夹 '{}' 已存在。", result_path.display());
        }

        let struct_file = File::create(result_path.join("struct.json"))?;
        serde_json::to_writer_pretty(struct_file, &struct_collector.structs)?;

        let enum_file = File::create(result_path.join("enum.json"))?;
        serde_json::to_writer_pretty(enum_file, &enum_collector.enums)?;

        let function_file = File::create(result_path.join("function.json"))?;
        serde_json::to_writer_pretty(function_file, &function_collector.functions)?;

        let union_file = File::create(result_path.join("union.json"))?;
        serde_json::to_writer_pretty(union_file, &union_collector.unions)?;

    }
    Ok(())
}