/*
Dollscript by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// containing the structure
/// responsible for catching
/// and handling errors.
pub mod err;

/// Declaring the module
/// containing unit tests
/// for the entities in
/// this crate.
#[cfg(test)]
pub mod tests;

/// Exporting the module
/// containing some utility
/// functions.
pub mod utils;

/// Exporting the module
/// containing entities
/// to tokenize Dollscript
/// source code and produce
/// a token stream. 
pub mod lexer;

/// Exporting the module
/// containing entities
/// for constructing a
/// syntax tree from a 
/// stream of tokens.
pub mod parser;
