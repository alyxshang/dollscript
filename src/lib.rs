/*
Dollscript by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// as a module.
pub mod modules;

/// Exporting the module
/// containing the structure
/// responsible for catching
/// and handling errors.
pub use modules::err::*;

/// Exporting the module
/// containing some utility
/// functions.
pub use modules::utils::*;

/// Exporting the module
/// containing entities
/// to tokenize Dollscript
/// source code and produce
/// a token stream. 
pub use modules::lexer::*;

