/*
Dollscript by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing the structure
/// responsible for catching
/// and handling errors.
pub use modules::err::*;

/// Re-exporting the module
/// containing some utility
/// functions.
pub use modules::utils::*;

/// Re-exporting the module
/// containing entities
/// to tokenize Dollscript
/// source code and produce
/// a token stream. 
pub use modules::lexer::*;

/// Re-exporting the module
/// containing entities
/// for constructing a
/// syntax tree from a 
/// stream of tokens.
pub use modules::parser::*;
