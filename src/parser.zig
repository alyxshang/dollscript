// Dollscript by Alyx Shang.
// Licensed under the FSL v1.

// Importing the "lexer" module
// to use the "Token" type.
const lexer = @import("lexer.zig");

// ATOMIC STATEMENTS
// * variable declaration
// * field declaration
// * struct declaration
// * conditional statement

/// A structure to parse a stream
/// of tokens obtained from Dollscript
/// code into a stream of nodes.
const Parser = struct {
    token_stream: ArrayList(Token),
};
