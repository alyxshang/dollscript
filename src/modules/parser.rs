/*
Dollscript by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// encapsulating data about
/// a captured token.
use super::lexer::Token;

/// Importing the enum
/// that "lists" all possible
/// tokens in a Dollscript
/// source string.
use super::lexer::TokenType;

/// Importing the structure for
/// catching and handling errors.
use super::err::DollscriptErr;

pub struct ImportStatement{
    pub path: String,
    pub alias: String
}

pub struct StructureDeclaration{
    pub fields: Vec<StructureField>
}

pub struct StructureField{
    pub name: String,
    pub field_type: String
}

pub struct VariableDeclaration{
    pub name: String,
    pub var_type: String,
    pub is_pointer: bool,
    pub is_mutable: bool
}

pub enum TopLevelStatement{
    Comment(String),
    Import(ImportStatement),
    FunctionDeclaration,
    StructureDeclaration(StructureDeclaration),

}

pub enum AtomicStatement{
    FunctionCall,
    AddStatement,
    DivisonStatement,
    MinusStatetement,
    MultiplicationStatement,
    VariableDeclaration(VariableDeclaration),
}

pub fn parse_token_stream(
    token_stream: &Vec<Token>
) -> Result<Vec<TopLevelStatement>, DollscriptErr>{
    let mut top_level_statements: Vec<TopLevelStatement> = Vec::new();
    let mut cursor: usize = 0;
    while cursor < token_stream.len(){
    }
    Ok(top_level_statements)
}
