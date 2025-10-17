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
    Colon, // used.
    Assign, // used.
    IsEqual, // used.
    NewLine, // used.
    PlusSign, // used.
    HashSign, // used.
    SemiColon, // used.
    OpenCurly, // used.
    UserIdent, // used.
    MinusSign, // used.
    TimesSign, // used.
    DivideSign, // used.
    UserString, // used.
    CloseCurly, // used.
    BooKeyword, // used.
    WhiteSpace, // used.
    OpenSquare, // used.
    CloseSquare, // used.
    FlexKeyword, // used.
    GlamKeyword, // used.
    UserComment, // used.
    GreaterThan, // used.
    SmallerThan, // used.
    OpenBracket, // used.
    BooleanTrue, // used.
    RockKeyword, // used.
    LoopArgument, // used.
    CaseOperator, // used.
    BooleanFalse, // used.
    LoopzKeyword, // used.
    CloseBracket, // used.
    IsTooKeyword, // used.
    FrostyKeyword, // used.
    ImportKeyword, // used.
    PublicKeyword, // used.
    WisdomKeyword, // used.
    IntegerNumber, // used.
    RangeOperator, // used.
    FloatingNumber,
    GreaterOrEqual, // used.
    SmallerOrEqual, // used.
    TernaryOperator, // used.
    FunctionKeyword, // used.
    SwitchupKeyword, // used.
    StructureKeyword, // used.
    NamespacingKeyword, // used.
    DoubleColonOperator, // used.
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
                let token: Token = Token::new( 
                    &(cursor+1), 
                    &cursor,
                    &None, 
                    &TokenType::Colon
                );
                stream.push(token);
                cursor += 1;
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
                    &TokenType::PlusSign
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
                let token: Token = Token::new(
                    &(cursor+1),
                    &cursor, 
                    &None, 
                    &TokenType::Assign
                );
                stream.push(token);
                cursor += 1;
            }
            else if current == '>'{
                let token: Token = Token::new(
                    &(cursor+1),
                    &cursor, 
                    &None, 
                    &TokenType::GreaterThan
                );
                stream.push(token);
                cursor += 1;
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
            else if current == '=' &&
                chars[cursor + 1] == '='
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+2), 
                    &None, 
                    &TokenType::IsEqual
                );
                stream.push(token);
                cursor += 2;
            }
            else if current == '>' &&
                chars[cursor + 1] == '='
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+2), 
                    &None, 
                    &TokenType::GreaterOrEqual
                );
                stream.push(token);
                cursor += 2;
            }
            else if current == '=' &&
                chars[cursor + 1] == '<'
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+2), 
                    &None, 
                    &TokenType::SmallerOrEqual
                );
                stream.push(token);
                cursor += 2;
            }
            else if current == '=' &&
                chars[cursor + 1] == '>'
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+2), 
                    &None, 
                    &TokenType::CaseOperator
                );
                stream.push(token);
                cursor += 2;
            }
            else if current == '.' &&
                chars[cursor + 1] == '.'
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
                chars[cursor + 1] == 'o' &&
                chars[cursor + 2] == 'o'
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
                chars[cursor + 1] == 'l' &&
                chars[cursor + 2] == 'e' &&
                chars[cursor + 3] == 'e'

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
                chars[cursor + 1] == 'l' &&
                chars[cursor + 2] == 'a' &&
                chars[cursor + 3] == 'm'

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
                chars[cursor + 1] == 'r' &&
                chars[cursor + 2] == 'u' &&
                chars[cursor + 3] == 'e'

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
                chars[cursor + 1] == 'o' &&
                chars[cursor + 2] == 'c' &&
                chars[cursor + 3] == 'k'

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
                chars[cursor + 1] == 'a' &&
                chars[cursor + 2] == 'l' &&
                chars[cursor + 3] == 's' &&
                chars[cursor + 4] == 'e'

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
                chars[cursor + 1] == 'o' &&
                chars[cursor + 2] == 'o' &&
                chars[cursor + 3] == 'p' &&
                chars[cursor + 4] == 'z'

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
                chars[cursor + 1] == 's' &&
                chars[cursor + 2] == 't' &&
                chars[cursor + 3] == 'o' &&
                chars[cursor + 4] == 'o'

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
                chars[cursor + 1] == 'r' &&
                chars[cursor + 2] == 'o' &&
                chars[cursor + 3] == 's' &&
                chars[cursor + 4] == 't' &&
                chars[cursor + 5] == 'y'

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
                chars[cursor + 1] == 'n' &&
                chars[cursor + 2] == 's' &&
                chars[cursor + 3] == 'p' &&
                chars[cursor + 4] == 'o'

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
                chars[cursor + 1] == 'l' &&
                chars[cursor + 2] == 'a' &&
                chars[cursor + 3] == 'y' &&
                chars[cursor + 4] == 'y'

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
                chars[cursor + 1] == 'i' &&
                chars[cursor + 2] == 's' &&
                chars[cursor + 3] == 'd' &&
                chars[cursor + 4] == 'o' &&
                chars[cursor + 5] == 'm'

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
                chars[cursor + 1] == 'o' &&
                chars[cursor + 2] == 'v' &&
                chars[cursor + 3] == 'e'
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
                chars[cursor + 1] == 'w' &&
                chars[cursor + 2] == 'i' &&
                chars[cursor + 3] == 't' &&
                chars[cursor + 4] == 'c' &&
                chars[cursor + 5] == 'h' &&
                chars[cursor + 6] == 'u' &&
                chars[cursor + 7] == 'p'
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
                chars[cursor + 1] == 'a' &&
                chars[cursor + 2] == 'g'
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+3), 
                    &None, 
                    &TokenType::SwitchupKeyword
                );
                stream.push(token);
                cursor += 3;
            }
            else if current == 'a' &&
                chars[cursor + 1] == 's'
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+2), 
                    &None, 
                    &TokenType::SwitchupKeyword
                );
                stream.push(token);
                cursor += 2;
            }
            else if current == ':' &&
                chars[cursor + 1] == ':'
            {
                let token: Token = Token::new(
                    &cursor, 
                    &(cursor+2), 
                    &None, 
                    &TokenType::DoubleColonOperator
                );
                stream.push(token);
                cursor += 2;
            }
            else if current == '<' &&
                chars[cursor + 1] == '3' &&
                chars[cursor + 2 ] == ' '
            {
                let start: usize = cursor.clone();
                cursor += 3;
                let mut str_buf: Vec<char> = Vec::new();
                while chars[cursor] != '\n' && cursor < chars.len(){
                    str_buf.push(chars[cursor]);
                    cursor += 1;
                }
                let token: Token = Token::new(
                    &start, 
                    &(cursor+2), 
                    &Some(str_buf.iter().collect::<String>()), 
                    &TokenType::UserComment
                );
                stream.push(token);
            }

            /*else if is_numeric(&current){
                let start: usize = cursor.clone();
                let mut str_buf: Vec<char> = Vec::new();
                while !is_numeric(&chars[cursor])  || chars[cursor] != '.' && cursor < chars.len(){
                    str_buf.push(chars[cursor]);
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
                        "Unexepcted token at position \"{:?}\"!", 
                        &cursor
                    );
                    return Err::<Vec<Token>, DollscriptErr>(
                        DollscriptErr::new(&e.to_string())
                    );
                }
            }*/
            else if current == '"'{
                let start: usize = cursor.clone();
                let mut str_buf: Vec<char> = Vec::new();
                while chars[cursor] != '"' &&
                    cursor < chars.len()
                {
                    str_buf.push(chars[cursor]);
                    cursor += 1;
                }
                let token: Token = Token::new(
                    &start, 
                    &(cursor+1), 
                    &Some(str_buf.iter().collect::<String>()), 
                    &TokenType::UserString
                );
                stream.push(token);
                cursor += 1;
            }
            else if is_alphabetic(&current){
                let start: usize = cursor.clone();
                let mut str_buf: Vec<char> = Vec::new();
                while !is_alphabetic(&chars[cursor]) && 
                    cursor < chars.len()
                {
                    str_buf.push(chars[cursor]);
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
