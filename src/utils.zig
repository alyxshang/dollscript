// Dollscript by Alyx Shang.
// Licensed under the FSL v1.

// Importing the standard
// library.
const std = @import("std")

// Importing the enum containing
// all possible error variants.
const err = @import("err.zig")

/// This function returns
/// the length of a string
/// with a null-terminator
/// as an unsigned integer.
pub export fn str_len(
    subject: [*:0]const u8
) usize {
    var len: usize = 0;
    var copy = subject;
    while (copy[0] != 0) {
        len += 1;
        copy += 1;
    }
    return len;
}

/// This function converts
/// a pointer to a null-terminated
/// array of characters into
/// an `ArrayList` of the same.
pub export fn to_array(
    subject: [*:0]const u8
) !ArrayList(u8) {
    const len: usize = str_len(subject);
    const allocator = std.heap.page_allocator;
    var char_list = std.ArrayList(Token).init(allocator);
    for i in 0..len{
        try char_list.append(subject[i]) 
            catch err.DollErr.CannotAppendErr;
    }
    return char_list;
}
