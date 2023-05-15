# Performance Mark

`performance_mark` is an attribute macro that adds performance (time) logging to
functions. By default, it uses `println!`, but can be configured to use a custom
method.

Basic example:

```rust
use performance_mark_attribute::performance_mark;

#[performance_mark]
fn test_function_logged_using_stdout() {
   println!("Hello!");
}
```

Output:

```
(performance_mark) test_function_logged_using_stdout took 7.177µs
```

## Custom Logging Method

```rust
use performance_mark_attribute::{performance_mark, LogContext}

#[performance_mark(log_with_this)]
fn test_function_logged_using_custom_function() {
   println!("Hello!");
}

fn log_with_this(ctx: LogContext) {
    println!("Function: {} , Time: {}ms", ctx.function, ctx.duration);
}
```

## Custom Async Logging Method

```rust
use performance_mark_attribute::{performance_mark, LogContext}

#[performance_mark(async log_with_this)]
fn test_function_logged_using_custom_function() {
   println!("Hello!");
}

async fn log_with_this(ctx: LogContext) {
    // Log asynchronously
}
```
