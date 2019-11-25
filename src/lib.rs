#[macro_use]
extern crate lazy_static;

pub mod compile;
pub use compile::*;
pub mod env;
pub use env::*;
pub mod ir;
pub use ir::*;
pub mod parser;
pub use parser::*;
pub mod simplify;
pub use simplify::*;
