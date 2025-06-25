pub mod compiler;
pub mod interpreter;
pub mod math;
pub mod parser;
pub mod repl;
pub mod runtime;
pub mod tests;
pub mod tokenizer;

// Re-export math module for easy access
pub use math::{MathModule, get_math_functions, get_math_constants};
