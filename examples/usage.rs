#[allow(unused_imports)]
use to_binary::{BinaryString,BinaryError};

fn main(){
    // A Demo For Working With Whitespace
    work_with_whitespace();

    // Generates A New BinaryString From Hexadecimal
    let x = generate();

    // Checks Whether The Input Is Binary
    let check_if_binary: bool = x.assert_binary();

    // Assert Input Is Binary
    assert_eq!(check_if_binary,true);

    // Retrieve Sizes Of Binary Input (Bits and Bytes)
    let size_in_bits = x.bits().unwrap();
    let size_in_bytes = x.bytes().unwrap();

    // Verifies Sizes Of Binary Inputs
    let verify_bit_length: bool = x.assert_bit_length(size_in_bits);
    let verify_byte_length: bool = x.assert_byte_length(size_in_bytes);

    // Assert Sizes Are Correct
    assert_eq!(verify_bit_length, true);
    assert_eq!(verify_byte_length, true);

}

fn generate() -> BinaryString {
    return BinaryString::from_hex("321155ED37271DE1A9C1914A92A5DFE4").unwrap()
}

fn work_with_whitespace(){
    // Generate From &str "Hello"
    let initial = BinaryString::from("Hello");

    // Add Spaces Between Each Byte In The `BinaryString` And Unwraps Result For Error-Checking Using `BinaryError`
    let spaces = initial.add_spaces().unwrap();

    // Removes All Whitespace In The `BinaryString`
    let removed_spaces = spaces.remove_spaces();

    // Asserts The Initial Result And The One With Removed Spaces Are The Same
    assert_eq!(initial,removed_spaces);
}