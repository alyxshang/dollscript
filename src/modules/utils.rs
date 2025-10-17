/*
Dollscript by Alyx Shang.
Licensed under the FSL v1.
*/

/// A function to check whether a character
/// is numeric or not. A boolean reflecting
/// this is returned.
pub fn is_numeric(sub: &char) -> bool {
    let alphabet: Vec<char> = "1234567890"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    alphabet.contains(sub)
}

/// A function to check whether a character
/// is alphabetic or not. A boolean reflecting
/// this is returned.
pub fn is_alphabetic(sub: &char) -> bool {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    alphabet.contains(sub)
}
