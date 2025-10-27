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

pub struct FunctionParameter{
    pub name: String,
    pub param_type: String
}

pub struct FunctionDeclaration{
    pub name: String,
    pub is_public: bool,
    pub return_type: String,
    pub body: Vec<AtomicStatement>,
    pub parameters: Vec<FunctionParameter>
}

pub struct FunctionCall{
    pub function_name: String,
    pub parameters: Vec<FunctionParameter>
}

pub struct AddStatement{
    pub right_hand_side: String,
    pub left_hand_side: String,
}

pub struct MinusStatement{
    pub right_hand_side: String,
    pub left_hand_side: String,
}

pub struct MulStatement{
    pub right_hand_side: String,
    pub left_hand_side: String,
}

pub struct DivStatement{
    pub right_hand_side: String,
    pub left_hand_side: String,
}

pub struct IfStatement{
    pub right_hand_side: String,
    pub left_hand_side: String,
    pub statements: Vec<AtomicStatement>
}

pub struct ElsifStatement{
    pub right_hand_side: String,
    pub left_hand_side: String,
    pub statements: Vec<AtomicStatement>
}

pub struct ElseStatement{
    pub statements: Vec<AtomicStatement>
}

pub enum TopLevelStatement{
    Comment(String),
    Import(ImportStatement),
    IfStatement(IfStatement),
    ElseStatement(ElseStatement),
    ElsifStatement(ElsifStatement),
    FunctionDeclaration(FunctionDeclaration),
    StructureDeclaration(StructureDeclaration),
}

pub enum AtomicStatement{
    FunctionCall(FunctionCall),
    AddStatement(AddStatement),
    DivStatement(DivStatement),
    MulStatement(MulStatement),
    MinusStatetement(MinusStatement),
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
