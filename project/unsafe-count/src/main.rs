use std::env;
use std::fs;
use std::path::Path;
//use syn::{visit::Visit, Item, ItemFn, Block, ExprUnsafe};
//use syn::{self, File, Item, ItemFn, ItemStruct, ItemUnion, ItemEnum, ItemStatic, ItemConst, ItemTrait, TraitItem, ItemImpl, Signature, Block, Stmt, Expr, ExprIf, ExprForLoop, ExprWhile, ExprMatch, ExprBlock, ExprLoop, ExprUnsafe, Label, Lifetime, ExprBreak, ExprContinue, Arm, Pat, parse_file};
use syn::{self, File, Item, ItemFn, ItemStruct, ItemUnion, ItemEnum, ItemStatic, ItemConst, ItemTrait, TraitItem, ItemImpl, Signature, Block, Stmt, Expr, ExprIf, ExprForLoop, ExprWhile, ExprMatch, ExprBlock, ExprLoop, ExprUnsafe, Label, Lifetime, Pat, parse_file};
use syn::spanned::Spanned;
use std::process;
use std::path::PathBuf;
use std::io;

struct UnsafeCounter {
    total_lines: usize,
    unsafe_lines: usize,
}

impl UnsafeCounter {
    fn new() -> Self {
        Self {
            total_lines: 0,
            unsafe_lines: 0,
        }
    }

    fn count_unsafe_in_fn(&mut self, item_fn: &ItemFn) {
        let start_line = item_fn.span().start().line;
        let end_line = item_fn.span().end().line;
        if item_fn.sig.unsafety.is_some() {
            self.unsafe_lines += end_line - start_line + 1;
            return; // If function is unsafe, we do not need to continue
        }
        
        self.process_block(&item_fn.block);
    }

    fn count_unsafe_in_trait(&mut self, item_trait: &ItemTrait) {
        let start_line = item_trait.span().start().line;
        let end_line = item_trait.span().end().line;
        if item_trait.unsafety.is_some() {
            self.unsafe_lines += end_line - start_line + 1;
            return; // If trait is unsafe, we do not need to continue
        }

        // Traverse all items in the trait to check for unsafe code
        for item in &item_trait.items {
            match item {
                TraitItem::Fn(method) => {
                    if method.sig.unsafety.is_some() {
                        let method_start_line = method.span().start().line;
                        let method_end_line = method.span().end().line;
                        self.unsafe_lines += method_end_line - method_start_line + 1;
                    }
                }
                _ => {}
            }
        }
        
    }

    fn count_unsafe_in_impl(&mut self, item_impl: &ItemImpl) {
        let start_line = item_impl.span().start().line;
        let end_line = item_impl.span().end().line;
        if item_impl.unsafety.is_some() {
            self.unsafe_lines += end_line - start_line + 1;
            return; // If impl block is unsafe, we do not need to continue
        }

        // Traverse all items in the impl block to check for unsafe code
        for item in &item_impl.items {
            match item {
                syn::ImplItem::Fn(method) => {
                    if method.sig.unsafety.is_some() {
                        let method_start_line = method.span().start().line;
                        let method_end_line = method.span().end().line;
                        self.unsafe_lines += method_end_line - method_start_line + 1;
                    } else {
                        self.process_block(&method.block);
                    }
                }
                _ => {}
            }
        }
    }

    // Handling statements within a code block
    fn process_block(&mut self, block: &Block) {

        for stmt in &block.stmts {
            match stmt {
                // When encountering control flow statements
                Stmt::Expr(Expr::If(_), _semi) | Stmt::Expr(Expr::While(_), _semi) |
                Stmt::Expr(Expr::ForLoop(_), _semi) | Stmt::Expr(Expr::Match(_), _semi) |
                Stmt::Expr(Expr::Loop(_), _semi) => {
                    self.process_expression(stmt);
                },
                Stmt::Expr(Expr::Unsafe(_), _semi) => {
                  let unsafe_start_line = stmt.span().start().line;
                  let unsafe_end_line = stmt.span().end().line;
                  self.unsafe_lines += unsafe_end_line - unsafe_start_line + 1;
                }
                // Add non control flow statements to the current block
                _ => {
                },
            }
        }
    }

    fn process_expression(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(Expr::If(expr_if), _semi) => {
                // Processing if expression
                self.process_if_expression(expr_if)
            },

            Stmt::Expr(Expr::ForLoop(expr_for), _semi) => {
                // Processing for loop expression
                self.process_for_loop_expression(expr_for)
            },

            Stmt::Expr(Expr::Match(expr_match), _semi) => {
                // Processing match expression
                self.process_match_expression(expr_match)
            },

            Stmt::Expr(Expr::While(expr_while), _semi) => {
                // Processing while expression
                self.process_while_expression(expr_while)
            },

            Stmt::Expr(Expr::Loop(expr_loop), _semi) => {
                // Processing loop expression
                self.process_loop_expression(expr_loop)
            },
            Stmt::Expr(Expr::Unsafe(_expr_unsafe), _semi) => {
                  let unsafe_start_line = stmt.span().start().line;
                  let unsafe_end_line = stmt.span().end().line;
                  self.unsafe_lines += unsafe_end_line - unsafe_start_line + 1;
            },
            _ => {
            }
        }
    }
    fn process_if_expression(&mut self, expr_if: &ExprIf) {
        // Processing if branches
        self.process_block(&expr_if.then_branch);
        if let Some((_else_token, else_branch)) = &expr_if.else_branch {
            match **else_branch {
                Expr::If(ref else_if_expr) => {
                    self.process_if_expression(else_if_expr);
                }

                Expr::Block(ref else_block) => {
                    self.process_block(&else_block.block);
                }
                _ => {
                    panic!("Unknown else branch!");
                }
            }
        } else {
        }
    }

    fn process_for_loop_expression(&mut self, expr_for: &ExprForLoop) {
        self.process_block(&expr_for.body);  // Processing for loop body
    }
    
    // Specifically for handling while loop
    fn process_while_expression(&mut self, expr_while: &ExprWhile) {
        self.process_block(&expr_while.body);
    }

    // Handling match expressions
    fn process_match_expression(&mut self, _expr_match: &ExprMatch) {
    }
    
    // Specifically for handling loop
    fn process_loop_expression(&mut self, expr_loop: &ExprLoop) {
        // Processing loop body
        self.process_block(&expr_loop.body);
    }

    fn process_file<P: AsRef<Path>>(&mut self, file_path: P) -> io::Result<()> {
        let content = fs::read_to_string(file_path).expect("Failed to read file");
        self.total_lines += content.lines().count();

        match parse_file(&content) {
            Ok(syntax_tree) => {
                self.count_unsafe_in_syntax_tree(syntax_tree);
            }
            Err(_) => {
                // 如果语法树解析失败，直接读取代码文件并统计
                self.count_unsafe_lines_in_content(&content);
            }
        }

        Ok(())
    }

    fn count_unsafe_in_syntax_tree(&mut self, syntax_tree: File) {
        for item in syntax_tree.items {
            match item {
                Item::Fn(item_fn) => {
                    self.count_unsafe_in_fn(&item_fn);
                }
                Item::Trait(item_trait) => {
                    self.count_unsafe_in_trait(&item_trait);
                }
                Item::Impl(item_impl) => {
                    self.count_unsafe_in_impl(&item_impl);
                }
                _ => {
                    // 其他类型的项不做处理
                }
            }
        }
    }

    fn count_unsafe_lines_in_content(&mut self, content: &str) {
        // 直接读取代码文件并统计包含 unsafe 关键字的行数
        for line in content.lines() {
            if line.contains("unsafe") {
                self.unsafe_lines += 1;
            }
        }
    }

    fn process_directory(&mut self, dir_path: &Path) -> std::io::Result<()> {
        if dir_path.is_dir() {
            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    // 递归处理子目录
                    self.process_directory(&path)?;
                } else if path.file_name() == Some(std::ffi::OsStr::new("build.rs"))
                    || path.file_name() == Some(std::ffi::OsStr::new("c2rust-lib.rs"))
                {
                    // 排除特定文件
                    continue
                }else if path.extension().map_or(false, |ext| ext == "rs") {
                    // 处理Rust源文件
                    self.process_file(&path);
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-rust-project>", args[0]);
        process::exit(1);
    }

    let project_path = PathBuf::from(&args[1]);
    let mut counter = UnsafeCounter::new();

    if let Err(e) = counter.process_directory(&project_path) {
        eprintln!("Error processing directory: {}", e);
        process::exit(1);
    }

    println!("Total lines: {}", counter.total_lines);
    println!("Unsafe lines: {}", counter.unsafe_lines);
    
    if counter.total_lines > 0 {
        let percentage = (counter.unsafe_lines as f64 / counter.total_lines as f64) * 100.0;
        println!("Unsafe percentage: {:.2}%", percentage);
    }
}
