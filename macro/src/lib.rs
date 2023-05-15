//! This crate implements the macro for `performance_mark` and should not be used directly.
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
/// performance_mark is an attribute macro that adds performance (time) logging to methods.
/// By default, it uses `println!`, but can be configured to use a custom method.
///
/// Basic example:
/// ```no_run no_test
/// use performance_mark::performance_mark;
///
/// #[performance_mark]
/// fn test_function_logged_using_stdout() {
///    println!("Hello!");
/// }
/// ```
///
/// Example with a custom logging method:
/// ```no_run no_test
/// use performance_mark::{performance_mark, LogContext}
///
/// #[performance_mark(log_with_this)]
/// fn test_function_logged_using_custom_function() {
///    println!("Hello!");
/// }
///
/// fn log_with_this(ctx: LogContext) {
///     println!("Function: {} , Time: {}ms", ctx.function, ctx.duration);
/// }
/// ```
///
/// Example with a custom async logging method:
/// ```no_run no_test
/// use performance_mark::{performance_mark, LogContext}
///
/// #[performance_mark(async log_with_this)]
/// fn test_function_logged_using_custom_function() {
///    println!("Hello!");
/// }
///
/// async fn log_with_this(ctx: LogContext) {
///     // Log asynchronously
/// }
/// ```
pub fn performance_mark(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as proc_macro2::TokenStream);
    let item = parse_macro_input!(item as proc_macro2::TokenStream);

    match performance_mark_impl::performance_mark(attr, item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
