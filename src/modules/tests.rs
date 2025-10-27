/*
Dollscript by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// for modelling a file
/// path in a cross-platform
/// way.
use std::path::PathBuf;

/// Importing the structure
/// encapsulating data about
/// a captured token.
use super::lexer::Token;

/// Importing the enum
/// that "lists" all possible
/// tokens in a Dollscript
/// source string.
use super::lexer::TokenType;

/// Importing the function
/// to tokenize a string of
/// Dollscript source code.
use super::lexer::tokenize;

/// Importing the function to
/// read the contents of a file
/// as a string.
use std::fs::read_to_string;

/// Importing the function
/// to check whether a character
/// is numeric.
use super::utils::is_numeric;

/// Importing the function
/// to check whether an ident 
/// contains valid characters.
use super::utils::is_alphabetic;

/// The function to test entities
/// inside the module containing
/// utility functions.
#[test]
pub fn test_utils(){
    let numeric_f: bool = is_numeric(&'c');
    let numeric_t: bool = is_numeric(&'7');
    let alphabetic_f: bool = is_alphabetic(&'8');
    let alphabetic_t_l: bool = is_alphabetic(&'c');
    let alphabetic_t_u: bool = is_alphabetic(&'C');
    assert_eq!(numeric_f, false);
    assert_eq!(numeric_t, true);
    assert_eq!(alphabetic_f, false);
    assert_eq!(alphabetic_t_l, true);
    assert_eq!(alphabetic_t_u, true);
}

/// The function to test entities
/// for tokenizing Dollscript
/// source code.
#[test]
pub fn test_lexer(){
    let test_string: String = r#":;,#()[]{}*/+-?=boo==flex=<glam=>true>=false<>..rockloopzistoofrostyinsposlayywisdommoveswitchupbagas::"Hello"45.6boo34my_ident"#
        .to_string();
    let tokenized: Vec<Token> = tokenize(&test_string)
        .expect("Unable to tokenize string.");
    let expected: Vec<Token> = vec![
        Token::new(&1,&0,&None,&TokenType::Colon), 
        Token::new(&2,&1,&None,&TokenType::SemiColon), 
        Token::new(&3,&2,&None,&TokenType::Comma), 
        Token::new(&4,&3,&None,&TokenType::HashSign), 
        Token::new(&5,&4,&None,&TokenType::OpenBracket), 
        Token::new(&6,&5,&None,&TokenType::CloseBracket), 
        Token::new(&7,&6,&None,&TokenType::OpenSquare), 
        Token::new(&8,&7,&None,&TokenType::CloseSquare), 
        Token::new(&9,&8,&None,&TokenType::OpenCurly), 
        Token::new(&10,&9,&None,&TokenType::CloseCurly), 
        Token::new(&11,&10,&None,&TokenType::TimesSign), 
        Token::new(&12,&11,&None,&TokenType::DivideSign), 
        Token::new(&13,&12,&None,&TokenType::PlusSign), 
        Token::new(&14,&13,&None,&TokenType::MinusSign), 
        Token::new(&15,&14,&None,&TokenType::TernaryOperator), 
        Token::new(&16,&15,&None,&TokenType::Assign), 
        Token::new(&16,&19,&None,&TokenType::BooKeyword), 
        Token::new(&21,&19,&None,&TokenType::IsEqual), 
        Token::new(&21,&25,&None,&TokenType::FlexKeyword), 
        Token::new(&27,&25,&None,&TokenType::SmallerOrEqual), 
        Token::new(&27,&31,&None,&TokenType::GlamKeyword), 
        Token::new(&33,&31,&None,&TokenType::CaseOperator), 
        Token::new(&33,&37,&None,&TokenType::BooleanTrue), 
        Token::new(&39,&37,&None,&TokenType::GreaterOrEqual), 
        Token::new(&39,&44,&None,&TokenType::BooleanFalse), 
        Token::new(&45,&44,&None,&TokenType::SmallerThan), 
        Token::new(&46,&45,&None,&TokenType::GreaterThan), 
        Token::new(&46,&48,&None,&TokenType::RangeOperator), 
        Token::new(&48,&52,&None,&TokenType::RockKeyword), 
        Token::new(&52,&57,&None,&TokenType::LoopzKeyword), 
        Token::new(&57,&62,&None,&TokenType::IsTooKeyword), 
        Token::new(&62,&68,&None,&TokenType::FrostyKeyword), 
        Token::new(&68,&73,&None,&TokenType::ImportKeyword), 
        Token::new(&73,&78,&None,&TokenType::PublicKeyword), 
        Token::new(&78,&84,&None,&TokenType::WisdomKeyword), 
        Token::new(&84,&88,&None,&TokenType::FunctionKeyword), 
        Token::new(&88,&96,&None,&TokenType::SwitchupKeyword), 
        Token::new(&96,&99,&None,&TokenType::StructureKeyword), 
        Token::new(&99,&101,&None,&TokenType::NamespacingKeyword), 
        Token::new(&103,&101,&None,&TokenType::DoubleColonOperator), 
        Token::new(&110,&103,&Some("Hello".to_string()),&TokenType::UserString), 
        Token::new(&110,&114,&Some("45.6".to_string()),&TokenType::FloatingNumber), 
        Token::new(&114,&117,&None,&TokenType::BooKeyword), 
        Token::new(&117,&119,&Some("34".to_string()),&TokenType::IntegerNumber), 
        Token::new(&127,&119,&Some("my_ident".to_string()),&TokenType::UserIdent)
    ];
    let extra_str: String = "my_ident\n<3 This is a comment.\n34"
        .to_string();
    let extra_str_tokenized: Vec<Token> = tokenize(&extra_str)
        .expect("Unable to tokenize extra string.");
    let extra_str_expected: Vec<Token> = vec![
        Token::new(&8,&0,&Some("my_ident".to_string()),&TokenType::UserIdent), 
        Token::new(&9,&8,&None,&TokenType::NewLine), 
        Token::new(&31,&9,&Some("<3 This is a comment.".to_string()),&TokenType::UserComment), 
        Token::new(&31,&33,&Some("34".to_string()),&TokenType::IntegerNumber)
    ];
    //let mut file_path_buf: PathBuf = PathBuf::new();
    //file_path_buf.push(env!("CARGO_MANIFEST_DIR"));
    //file_path_buf.push("sample/sample.doll");
    //let sample_str: String = read_to_string(&file_path_buf.display().to_string())
    //    .expect("Could not read file.");
    //let sample_tokenized: Vec<Token> = tokenize(&sample_str)
    //    .expect("Could not tokenize sample string.");
    //println!("{:?}", sample_tokenized);
    assert_eq!(tokenized, expected);
    assert_eq!(extra_str_tokenized, extra_str_expected);
}
