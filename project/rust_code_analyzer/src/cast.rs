use syn::{Expr, File};
use syn::visit_mut::{self, VisitMut};

struct EnumCastRewriter {
    enum_name: String,
}

impl VisitMut for EnumCastRewriter {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        // 避免访问字段链式调用后触发 panic
        if let Expr::Cast(cast) = expr {
            // 检查 cast.expr 是否是 Field，避免替换 (x as Enum).field 的情况
            if matches!(*cast.expr, Expr::Field(_)) {
                return; // 跳过这个表达式
            }
    
            if let syn::Type::Path(type_path) = &*cast.ty {
                if let Some(ident) = type_path.path.get_ident() {
                    if ident == &self.enum_name {
                        let enum_ident = ident.clone();
                        let expr_inner = &cast.expr;
                        let new_expr: Expr = syn::parse_quote! {
                            #enum_ident::from_libc_c_uint(#expr_inner as u32)
                        };
                        *expr = new_expr;
                    }
                }
            }
        }
    
        // 递归调用放到最后，确保我们修改 expr 后不再继续递归访问它
        visit_mut::visit_expr_mut(self, expr);
    }    
}

pub fn rewrite_casts(code: &str, enum_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut syntax: File = syn::parse_file(code)?;
    let mut rewriter = EnumCastRewriter {
        enum_name: enum_name.to_string(),
    };
    rewriter.visit_file_mut(&mut syntax);
    Ok(prettyplease::unparse(&syntax))
}


