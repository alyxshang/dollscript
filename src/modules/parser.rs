/*
Dollscript by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// encapsulating data about
/// a captured token.
use super::lexer::Token;

/// Importing the structure for
/// catching and handling errors.
use super::err::DollscriptErr;

pub enum StatementType{
    Import{
        path: String,
        alias: String
    },


}

pub fn parse_token_stream(
    token_stream: &Vec<Token>
) -> Result<(), DollscriptErr>{
}
