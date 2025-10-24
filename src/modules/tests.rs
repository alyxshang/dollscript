/*
Dollscript by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// encapsulating data about
/// a captured token.
use super::lexer::Token;

/// Importing the function
/// to tokenize a string of
/// Dollscript source code.
use super::lexer::tokenize;

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
    let test_string: String = ":;,#()[]{}*+-/\n?=><|==>==<=>..booflexglamrockfalseloopzistoofrostyinsposlayywisdommoveswitchupbagas::<3Comment here.\n345\"Hello\"myVariable"
        .to_string();
    let tokenized: Vec<Token> = tokenize(&test_string)
        .expect("Unable to tokenize string.");
    println!("{:?}", tokenized);
}
