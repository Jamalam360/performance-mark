use std::time::Duration;

#[doc(inline)]
pub use performance_mark_macro::performance_mark;

/// Context passed to a custom logging function.
pub struct LogContext {
    /// The name of the function being profiled.
    pub function: String,
    /// The time the function took to complete.
    pub duration: Duration,
}
