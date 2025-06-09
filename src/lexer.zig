// Dollscript by Alyx Shang.
// Licensed under the FSL v1.

// Importing the standard
// library.
const std = @import("std")

// Importing the enum containing
// all possible error variants.
const err = @import("err.zig")

// Importing the enum containing
// some utility functions.
const utils = @import("utils.zig")

/// An enum containing all possible
/// types a token in Dollscript
/// code can have.
pub export TokenType = enum(u8) {
    TernaryOperator, // ? // used
    DollarKeyword // String interpolation // used
    InspoKeyword, // import. // used
    SlayKeyword, // public // used
    OpenCurly, // { // used
    CloseCurly, // } // used
    BagKeyword, // struct // used
    StringLiteral, // "doll" // used
    UserType, // any structs  // used
    Colon, // : // used
    OpenBracket, // ( // used
    CloseBracket, // ) // used
    WisdomKeyword, // const // used
    BooKeyword, // var // used
    EqualsOperator, // == // used
    PlusOperator, // + // used
    MultplicationOperator, // * // used
    DivisionOperator, // /
    VerticalLine, // | // used
    LoopzKeyword, // for // used
    InKeyword, // in // used
    IntegerNumber, // cash // used
    FloatNumber, // change // used
    TrueKeyword, // true // used
    FalseKeyword, // false // used
    RangeOperator, // ~ // used
    AssignOperator, // = // used.
    SmallerThanOperator, // <
    MatchArmKeyword, // => // used
    GreaterThanOperator, // > // used
    SmallerOrEqualOperator, // =<
    GreaterOrEqualOperator, // >=
    IsTooOperator, // else if // used
    GlamCheckOperator, // if // used
    StyleSwitchOperator, // switch/match // used
    MoveKeyword, // function // used
    Frosty // else // used.
    OpenSquare, // [
    CloseSquare, // ]
    RockKeyword, // while // used
    GirlWaitOperator, // async
}

/// A structure to store info
/// on a lexed token.
pub export Token = struct {
    end: i32,
    start: i32,
    value: [*:0]const u8,
    token_type: TokenType,

    /// Initiates a new
    /// instance of the
    /// `Token` structure.
    pub fn export init(
        end: i32,
        start: i32,
        value: [*:0]const u8,
        token_type: TokenType
    ) Token {
        return Token{
            .end = end,
            .start = start,
            .value = value,
            .token_type = token_type
        };
    }
}


/// A structure to save a stream
/// of tokens and lex the input
/// string into a stream of
/// tokens.
pub export Lexer = struct {
    source: [*:0]const u8,
    token_stream: ArrayList(Token),

    /// Initiates a new
    /// instance of the
    /// `Lexer` structure.
    pub export fn init(
        source: [*:0]const u8, 
    ) Lexer {
        const allocator = std.heap.page_allocator;
        var stream = std.ArrayList(Token).init(allocator);
        return Lexer {
            .source = source,
            .token_stream = stream
        };
    }

    /// Lexes the `source`
    /// field into a stream
    /// of tokens.
    pub export fn lex() void {
        const len = str_len(@this.source);
        var array: ArrayList(Token) = ArrayList(Token)
            .init(allocator);
        for (0..len) |i| {

        }
    }
}
