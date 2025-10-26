/*
Dollscript by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// to check whether a character
/// is numeric or not.
use super::utils::is_numeric;

/// Importing the structure for
/// catching and handling errors.
use super::err::DollscriptErr;

/// Importing the function
/// to check whether a character
/// is alphabetic or not.
use super::utils::is_alphabetic;

/// The enumeration "listing"
/// every possible type of
/// token used in Dollscript
/// source code.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenType{
    Colon,
    Comma, 
    Assign,
    IsEqual,
    NewLine,
    PlusSign,
    HashSign,
    SemiColon,
    OpenCurly,
    UserIdent,
    MinusSign,
    TimesSign,
    DivideSign,
    UserString,
    CloseCurly,
    BooKeyword,
    WhiteSpace,
    OpenSquare,
    CloseSquare,
    FlexKeyword,
    GlamKeyword,
    UserComment,
    GreaterThan,
    SmallerThan,
    OpenBracket,
    BooleanTrue,
    RockKeyword,
    LoopArgument,
    CaseOperator,
    BooleanFalse,
    LoopzKeyword,
    CloseBracket,
    IsTooKeyword, 
    FrostyKeyword,
    ImportKeyword,
    PublicKeyword,
    WisdomKeyword,
    IntegerNumber,
    RangeOperator,
    FloatingNumber,
    GreaterOrEqual,
    SmallerOrEqual,
    TernaryOperator,
    FunctionKeyword,
    SwitchupKeyword,
    StructureKeyword,
    NamespacingKeyword, 
    DoubleColonOperator,
}

/// A structure to encapsulate
/// data about a captured token.
#[derive(PartialEq, Debug, Clone)]
pub struct Token{
    pub end: usize,
    pub start: usize,
    pub value: Option<String>,
    pub token_type: TokenType
}

/// Implementing functions for
/// this structure.
impl Token {

    /// A function to create a new
    /// instance of this structure
    /// and return it.
    pub fn new(
        end: &usize,
        start: &usize,
        value: &Option<String>,
        token_type: &TokenType
    ) -> Token {
        Token{
            end: *end,
            start: *start,
            value: value.clone(),
            token_type: token_type.clone()
        }
    }

}

/// Attempts to capture a stream of tokens
/// from a supplied string of Dollscript source
/// code. If the operation is successful, a vector
/// of instances of the `Token` struct is returned.
/// If the operation fails, an error is returned.
pub fn tokenize(
    src: &str
) -> Result<Vec<Token>, DollscriptErr>{
    let chars: Vec<char> = src
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    if chars.len() == 0{
        Err::<Vec<Token>, DollscriptErr>(
            DollscriptErr::new("Source string cannot be empty.")
        )
    }
    else {
        let mut stream: Vec<Token> = Vec::new();
        let mut cursor: usize = 0;
        while cursor < chars.len(){
            let current: char = chars[cursor];
            if current == ':'{
                if chars.get(cursor + 1) == Some(&':'){
                    let token: Token = Token::new( 
                        &(cursor+2), 
                        &cursor,
                        &None, 
                        &TokenType::DoubleColonOperator
                    );
                    stream.push(token);
                    cursor += 2;
                }
                else {
                    let token: Token = Token::new( 
                        &(cursor+1), 
                        &cursor,
                        &None, 
                        &TokenType::Colon
                    );
                    stream.push(token);
                    cursor += 1;
                }
            }
            else if current == ';'{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::SemiColon
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == ','{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::Comma
                );
                stream.push(token);
                cursor += 1;
            }

            else if current == '#'{
                let token: Token = Token::new( 
                    &(cursor+1), 
                    &cursor,
                    &None, 
                    &TokenType::HashSign
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '('{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::OpenBracket
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == ')'{
                let token: Token = Token::new(
                    &(cursor+1), 
                    &cursor,
                    &None, 
                    &TokenType::CloseBracket
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '['{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::OpenSquare
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == ']'{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::CloseSquare
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '{'{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::OpenCurly
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '}'{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::CloseCurly
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '*'{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::TimesSign
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '+'{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::PlusSign
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '-'{
                let token: Token = Token::new(
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::MinusSign
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '/'{
                let token: Token = Token::new( 
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::DivideSign
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '\n'{
                let token: Token = Token::new(
                    &(cursor+1),
                    &cursor, 
                    &None, 
                    &TokenType::NewLine
                );
                stream.push(token);
                cursor += 1;
            }
           else if current == ' '{
                let token: Token = Token::new(
                    &(cursor+1),
                    &cursor, 
                    &None, 
                    &TokenType::WhiteSpace
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '?'{
                let token: Token = Token::new(
                    &(cursor+1),
                    &cursor, 
                    &None, 
                    &TokenType::TernaryOperator
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '='{
                if chars.get(cursor+1) == Some(&'='){
                    let token: Token = Token::new(
                        &(cursor+2),
                        &cursor, 
                        &None, 
                        &TokenType::IsEqual
                    );
                    stream.push(token);
                    cursor += 2;
                }
                else if chars.get(cursor+1) == Some(&'>'){
                    let token: Token = Token::new(
                        &(cursor+2),
                        &cursor, 
                        &None, 
                        &TokenType::CaseOperator
                    );
                    stream.push(token);
                    cursor += 2;
                }
                else if chars.get(cursor+1) == Some(&'<'){
                    let token: Token = Token::new(
                        &(cursor+2),
                        &cursor, 
                        &None, 
                        &TokenType::SmallerOrEqual
                    );
                    stream.push(token);
                    cursor += 2;
                }
                else {
                    let token: Token = Token::new(
                        &(cursor+1),
                        &cursor, 
                        &None, 
                        &TokenType::Assign
                    );
                    stream.push(token);
                    cursor += 1;
                }
            }
            else if current == '>'{
                if chars[cursor+1] == '='{
                    let token: Token = Token::new(
                        &(cursor+2),
                        &cursor, 
                        &None, 
                        &TokenType::GreaterOrEqual
                    );
                    stream.push(token);
                    cursor += 2;
                }
                else {
                    let token: Token = Token::new(
                        &(cursor+1),
                        &cursor, 
                        &None, 
                        &TokenType::GreaterThan
                    );
                    stream.push(token);
                    cursor += 1;
                }
            }
            else if current == '<'{
                let token: Token = Token::new(
                    &(cursor+1),
                    &cursor, 
                    &None, 
                    &TokenType::SmallerThan
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '|'{
                let token: Token = Token::new(
                    &(cursor+1),
                    &cursor,
                    &None, 
                    &TokenType::LoopArgument
                );
                stream.push(token);
                cursor += 1;
            } 
            else if current == '.' &&
                chars.get(cursor + 1) == Some(&'.')
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+2), 
                    &None, 
                    &TokenType::RangeOperator
                );
                stream.push(token);
                cursor += 2;
            }
            else if current == 'b' &&
                chars.get(cursor + 1) == Some(&'o') &&
                chars.get(cursor + 2) == Some(&'o')
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+3), 
                    &None, 
                    &TokenType::BooKeyword
                );
                stream.push(token);
                cursor += 3;
            }
            else if current == 'f' &&
                chars.get(cursor + 1) == Some(&'l') &&
                chars.get(cursor + 2) == Some(&'e') &&
                chars.get(cursor + 3) == Some(&'x')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+4), 
                    &None, 
                    &TokenType::FlexKeyword
                );
                stream.push(token);
                cursor += 4;
            }
            else if current == 'g' &&
                chars.get(cursor + 1) == Some(&'l') &&
                chars.get(cursor + 2) == Some(&'a') &&
                chars.get(cursor + 3) == Some(&'m')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+4), 
                    &None, 
                    &TokenType::GlamKeyword
                );
                stream.push(token);
                cursor += 4;
            }
            else if current == 't' &&
                chars.get(cursor + 1) == Some(&'r') &&
                chars.get(cursor + 2) == Some(&'u') &&
                chars.get(cursor + 3) == Some(&'e')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+4), 
                    &None, 
                    &TokenType::BooleanTrue
                );
                stream.push(token);
                cursor += 4;
            }
            else if current == 'r' &&
                chars.get(cursor + 1) == Some(&'o') &&
                chars.get(cursor + 2) == Some(&'c') &&
                chars.get(cursor + 3) == Some(&'k')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+4), 
                    &None, 
                    &TokenType::RockKeyword
                );
                stream.push(token);
                cursor += 4;
            }
            else if current == 'f' &&
                chars.get(cursor + 1) == Some(&'a') &&
                chars.get(cursor + 2) == Some(&'l') &&
                chars.get(cursor + 3) == Some(&'s') &&
                chars.get(cursor + 4) == Some(&'e')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+5), 
                    &None, 
                    &TokenType::BooleanFalse
                );
                stream.push(token);
                cursor += 5;
            }
            else if current == 'l' &&
                chars.get(cursor + 1) == Some(&'o') &&
                chars.get(cursor + 2) == Some(&'o') &&
                chars.get(cursor + 3) == Some(&'p') &&
                chars.get(cursor + 4) == Some(&'z')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+5), 
                    &None, 
                    &TokenType::LoopzKeyword
                );
                stream.push(token);
                cursor += 5;
            }
            else if current == 'i' &&
                chars.get(cursor + 1) == Some(&'s') &&
                chars.get(cursor + 2) == Some(&'t') &&
                chars.get(cursor + 3) == Some(&'o') &&
                chars.get(cursor + 4) == Some(&'o')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+5), 
                    &None, 
                    &TokenType::IsTooKeyword
                );
                stream.push(token);
                cursor += 5;
            }
            else if current == 'f' &&
                chars.get(cursor + 1) == Some(&'r') &&
                chars.get(cursor + 2) == Some(&'o') &&
                chars.get(cursor + 3) == Some(&'s') &&
                chars.get(cursor + 4) == Some(&'t') &&
                chars.get(cursor + 5) == Some(&'y')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+6), 
                    &None, 
                    &TokenType::FrostyKeyword
                );
                stream.push(token);
                cursor += 6;
            }
            else if current == 'i' &&
                chars.get(cursor + 1) == Some(&'n') &&
                chars.get(cursor + 2) == Some(&'s') &&
                chars.get(cursor + 3) == Some(&'p') &&
                chars.get(cursor + 4) == Some(&'o')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+5), 
                    &None, 
                    &TokenType::ImportKeyword
                );
                stream.push(token);
                cursor += 5;
            }
            else if current == 's' &&
                chars.get(cursor + 1) == Some(&'l') &&
                chars.get(cursor + 2) == Some(&'a') &&
                chars.get(cursor + 3) == Some(&'y') &&
                chars.get(cursor + 4) == Some(&'y')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+5), 
                    &None, 
                    &TokenType::PublicKeyword
                );
                stream.push(token);
                cursor += 5;
            }
            else if current == 'w' &&
                chars.get(cursor + 1) == Some(&'i') &&
                chars.get(cursor + 2) == Some(&'s') &&
                chars.get(cursor + 3) == Some(&'d') &&
                chars.get(cursor + 4) == Some(&'o') &&
                chars.get(cursor + 5) == Some(&'m')

            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+6), 
                    &None, 
                    &TokenType::WisdomKeyword
                );
                stream.push(token);
                cursor += 6;
            }
            else if current == 'm' &&
                chars.get(cursor + 1) == Some(&'o') &&
                chars.get(cursor + 2) == Some(&'v') &&
                chars.get(cursor + 3) == Some(&'e')
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+4), 
                    &None, 
                    &TokenType::FunctionKeyword
                );
                stream.push(token);
                cursor += 4;
            }
            else if current == 's' &&
                chars.get(cursor + 1) == Some(&'w') &&
                chars.get(cursor + 2) == Some(&'i') &&
                chars.get(cursor + 3) == Some(&'t') &&
                chars.get(cursor + 4) == Some(&'c') &&
                chars.get(cursor + 5) == Some(&'h') &&
                chars.get(cursor + 6) == Some(&'u') &&
                chars.get(cursor + 7) == Some(&'p')
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+8), 
                    &None, 
                    &TokenType::SwitchupKeyword
                );
                stream.push(token);
                cursor += 8;
            }
            else if current == 'b' &&
                chars.get(cursor + 1) == Some(&'a') &&
                chars.get(cursor + 2) == Some(&'g')
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+3), 
                    &None, 
                    &TokenType::StructureKeyword
                );
                stream.push(token);
                cursor += 3;
            }
            else if current == 'a' &&
                chars.get(cursor + 1) == Some(&'s')
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+2), 
                    &None, 
                    &TokenType::NamespacingKeyword
                );
                stream.push(token);
                cursor += 2;
            } 
            else if is_numeric(&current){
                let start: usize = cursor.clone();
                let mut str_buf: Vec<char> = Vec::new();
                while let Some(&c) = chars.get(cursor){
                    println!("{}", &c);
                    if is_numeric(&c){
                        str_buf.push(c);
                    }
                    else if c == '.' {
                        str_buf.push(c);
                    }
                    else {
                        break;
                    }
                    cursor += 1;
                }
                let joined: String = str_buf
                    .iter()
                    .collect::<String>();
                let split: Vec<&str> = joined
                    .split(".")
                    .into_iter()
                    .collect::<Vec<&str>>();
                if split.len() == 1 && !joined.contains("."){
                    let token: Token = Token::new(
                        &start, 
                        &(cursor), 
                        &Some(split[0].to_string()), 
                        &TokenType::IntegerNumber
                    );
                    stream.push(token);  
                }
                else if split.len() == 2{
                    let token: Token = Token::new(
                        &start, 
                        &(cursor), 
                        &Some(str_buf.iter().collect::<String>()), 
                        &TokenType::FloatingNumber
                    );
                    stream.push(token);  
                }
                else {
                    let e: String = format!(
                        "Unexpcted token at position \"{:?}\"!", 
                        &cursor
                    );
                    return Err::<Vec<Token>, DollscriptErr>(
                        DollscriptErr::new(&e.to_string())
                    );
                }
            }
            else if current == '"'{
                let start: usize = cursor.clone();
                let mut str_buf: Vec<char> = Vec::new();
                cursor += 1;
                while chars.get(cursor) != Some(&'"')
                {
                    match chars.get(cursor){
                        Some(c) => str_buf.push(*c),
                        None => {}
                    };
                    cursor += 1;
                }
                let token: Token = Token::new(
                    &(cursor+1), 
                    &start, 
                    &Some(str_buf.iter().collect::<String>()), 
                    &TokenType::UserString
                );
                stream.push(token);
                cursor += 1;
            }
            else if is_alphabetic(&current){
                let start: usize = cursor.clone();
                let mut str_buf: Vec<char> = Vec::new();
                while let Some(&c) = chars.get(cursor){
                    if is_alphabetic(&c){
                        str_buf.push(c);
                    }
                    else {
                        break;
                    }
                    cursor += 1;
                }
                let token: Token = Token::new(
                    &start, 
                    &(cursor), 
                    &Some(str_buf.iter().collect::<String>()), 
                    &TokenType::UserIdent
                );
                stream.push(token);
            }
            else {
                let e: String = format!(
                    "Unexepcted token at position \"{:?}\"!", 
                    &cursor
                );
                return Err::<Vec<Token>, DollscriptErr>(
                    DollscriptErr::new(&e.to_string())
                );
            }
        }
        Ok(stream)
    }
}
