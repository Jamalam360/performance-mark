//! This crate implements the macro for `performance_mark` and should not be used directly.

use proc_macro2::{TokenStream, TokenTree, Ident};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse2, parse_quote,
    spanned::Spanned,
    visit_mut::{visit_stmt_mut, VisitMut},
    Expr, Item, Stmt,
};

#[doc(hidden)]
pub fn performance_mark(attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
    let mut item = match parse2::<Item>(item.clone()).unwrap() {
        Item::Fn(function) => function,
        _ => {
            return Err(syn::Error::new(
                item.into_token_stream().span(),
                "Unexpected token",
            ))
        }
    };

    let mut asyncness = false;
    let mut log_function = None;

    for token in attr {
        match token {
            TokenTree::Ident(ref ident) => {
                if ident.to_string() == "async" {
                    asyncness = true;
                } else if log_function.is_none() {
                    log_function = Some(ArbitraryFunction(ident.clone()))
                } else {
                    return Err(syn::Error::new(token.span(), "Unexpected token"));
                }
            }
            _ => return Err(syn::Error::new(token.span(), "Unexpected token")),
        }
    }

    let start_stmt: Stmt = parse_quote!(let start = std::time::Instant::now(););

    item.block.stmts.insert(0, start_stmt);

    let function_name = item.sig.ident.to_string();
    let mut end_stmts: Vec<Stmt> = parse_quote! {
        let ctx = performance_mark::LogContext {
            function: #function_name.to_string(),
            duration: std::time::Instant::now().duration_since(start),
        };
    };

    if let Some(log_function) = log_function {
        if asyncness {
            end_stmts.push(parse_quote! {
                #log_function(ctx).await;
            })
        } else {
            end_stmts.push(parse_quote! {
                #log_function(ctx);
            });
        }
    } else {
        end_stmts.push(parse_quote! {
            println!("(performance_mark) {} took {:?}", ctx.function, ctx.duration);
        })
    }

    let mut visitor = InsertBeforeReturnVisitor {
        end_stmts: &end_stmts,
        asyncness,
    };
    visitor.visit_item_fn_mut(&mut item);
    item.block.stmts.extend(end_stmts);

    Ok(item.into_token_stream())
}

struct InsertBeforeReturnVisitor<'a> {
    end_stmts: &'a Vec<Stmt>,
    asyncness: bool,
}

impl<'a> InsertBeforeReturnVisitor<'a> {
    fn construct_expr(&self, return_stmt: &Stmt) -> Expr {
        let stmts = VecStmt(self.end_stmts);

        if self.asyncness {
            Expr::Await(parse_quote! {
                async {
                    #stmts
                    #return_stmt
                }.await
            })
        } else {
            Expr::Block(parse_quote! {
                {
                    #stmts,
                    #return_stmt
                }
            })
        }
    }
}

impl<'a> VisitMut for InsertBeforeReturnVisitor<'a> {
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        let original_stmt = stmt.clone();

        match stmt {
            Stmt::Expr(Expr::Return(return_expr), _) => {
                return_expr
                    .expr
                    .replace(Box::new(self.construct_expr(&original_stmt)));
            }
            Stmt::Expr(ref mut return_expr, None) => {
                match return_expr {
                    Expr::ForLoop(_) | Expr::If(_) | Expr::Loop(_) | Expr::While(_) => {
                        return visit_stmt_mut(self, stmt);
                    }
                    _ => {}
                }

                *return_expr = self.construct_expr(&original_stmt);
            }
            _ => {}
        }
    }

    fn visit_expr_closure_mut(&mut self, _: &mut syn::ExprClosure) {
        // NO-OP, do not visit the inside of closures
    }
}

struct VecStmt<'a>(&'a Vec<Stmt>);

impl<'a> ToTokens for VecStmt<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for stmt in self.0.iter() {
            stmt.to_tokens(tokens);
        }
    }
}

struct ArbitraryFunction(Ident);

impl ToTokens for ArbitraryFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.0.clone());
    }
}
