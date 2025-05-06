extern crate syn;
extern crate quote;
use syn::visit_mut::{self, VisitMut};
use syn::parse_quote;
use crate::quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{File, Item, Type, FnArg, PatType, ItemFn, Ident, TypePtr, TypeReference, ItemImpl, ItemStruct, Field};
use std::collections::HashSet;

fn collect_struct_names(syntax: &File) -> HashSet<String> {
    syntax.items.iter().filter_map(|item| {
        if let Item::Struct(s) = item {
            Some(s.ident.to_string())
        } else {
            None
        }
    }).collect()
}

fn should_transform_to_method(func: &syn::ItemFn, struct_names: &HashSet<String>) -> Option<syn::Ident> {
    for input in &func.sig.inputs {
        if let FnArg::Typed(PatType { ty, .. }) = input {
            if let Type::Path(type_path) = &**ty {
                if let Some(ident) = type_path.path.segments.last() {
                    if struct_names.contains(&ident.ident.to_string()) {
                        return Some(ident.ident.clone());
                    }
                }
            }
        }
    }
    None
}

/// 替换 libc 类型为原生 Rust 类型
struct TypeRewriter;

impl VisitMut for TypeRewriter {
    fn visit_type_mut(&mut self, node: &mut Type) {

        if let Type::Path(ref mut type_path) = node {
            if let Some(last_segment) = type_path.path.segments.last() {
                let type_name = last_segment.ident.to_string();
                *node = match type_name.as_str() {
                    "c_int" => syn::parse_quote!(i32),
                    "c_uint" => syn::parse_quote!(u32),
                    "c_char" => syn::parse_quote!(i8),
                    "c_uchar" => syn::parse_quote!(u8),
                    "c_long" => syn::parse_quote!(i64),
                    "c_ulong" => syn::parse_quote!(u64),
                    _ => node.clone(),
                };
            }
        }
        visit_mut::visit_type_mut(self, node);
    }

    fn visit_field_mut(&mut self, field: &mut Field) {
        // 这里处理结构体字段的类型
        self.visit_type_mut(&mut field.ty);  // 修改字段类型
    }

    fn visit_item_struct_mut(&mut self, item: &mut ItemStruct) {
        // 这里遍历结构体的字段
        for field in &mut item.fields {
            self.visit_field_mut(field);
        }
    }
}

/// 提取并重构与结构体关联的函数为 impl 方法
fn convert_function_to_method(func: &ItemFn, struct_ident: &Ident) -> Option<syn::ItemImpl> {
    let inputs = func.sig.inputs.clone().into_iter();
    let mut found_self = false;
    let mut new_inputs = Punctuated::new();

    // 处理函数参数，查找结构体类型的参数，并替换为 self 或 &self
    for input in inputs {
        if let FnArg::Typed(PatType { ty, .. }) = &input {
            if let Type::Path(type_path) = &**ty {
                if let Some(seg) = type_path.path.segments.last() {
                    if seg.ident == *struct_ident {
                        // 处理 self 参数
                        let self_arg: FnArg = match &**ty {
                            Type::Path(_) => parse_quote!(mut self),  // 处理 `self`
                            Type::Reference(TypeReference { mutability, .. }) => {
                                if mutability.is_some() {
                                    parse_quote!(&mut self)  // 处理 `&mut self`
                                } else {
                                    parse_quote!(&self)  // 处理 `&self`
                                }
                            }
                            Type::Ptr(TypePtr { mutability, .. }) => {
                                if mutability.is_some() {
                                    parse_quote!(&mut self)  // 处理 `&mut self`
                                } else {
                                    parse_quote!(&self)  // 处理 `&self`
                                }
                            }
                            _ => parse_quote!(self),  // 默认使用 `self`
                        };
                        new_inputs.push(self_arg);
                        found_self = true;
                        continue;  // 跳过当前参数，处理下一个
                    }
                }
            }
        }
        new_inputs.push(input);  // 保持其他参数不变
    }

    // 如果没有找到 self 参数，返回 None
    if !found_self {
        return None;
    }

    // 克隆原始函数并修改其参数
    let mut method = func.clone();
    method.sig.inputs = new_inputs;

    // 构造 ItemFn（表示方法）
    let method_item: syn::ItemFn = parse_quote! {
        fn #method
    };

    // 构造 impl 块
    let impl_block: ItemImpl = parse_quote! {
        impl #struct_ident {
            #method_item
        }
    };

    Some(impl_block)
}

fn main() {
    use std::io::{self, Read};

    // 从 stdin 读取代码
    let mut code = String::new();
    io::stdin().read_to_string(&mut code).expect("Failed to read code from stdin");

    let mut syntax: File = syn::parse_file(&code).expect("Failed to parse Rust code");

    let mut type_rewriter = TypeRewriter;
    type_rewriter.visit_file_mut(&mut syntax);

    // 搜集结构体名和函数项
    let mut impls = vec![];
    let mut retained_items = vec![];

    let struct_names = collect_struct_names(&syntax);

    for item in syntax.items {
        match item {
            Item::Fn(func) => {
                if let Some(struct_ident) = should_transform_to_method(&func, &struct_names) {
                    if let Some(imp) = convert_function_to_method(&func, &struct_ident) {
                        impls.push(Item::Impl(imp));
                    } else {
                        retained_items.push(Item::Fn(func));
                    }
                } else {
                    retained_items.push(Item::Fn(func));
                }
            }
            other => retained_items.push(other),
        }
    }

    retained_items.extend(impls);
    let new_file = File {
        shebang: syntax.shebang,
        attrs: syntax.attrs,
        items: retained_items,
    };

    println!("{}", prettyplease::unparse(&new_file));
}