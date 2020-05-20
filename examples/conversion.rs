use to_binary::BinaryString;

fn main() {
    hexadecimal();
    str_func();
    string();
    vector();
    byte();
}

fn hexadecimal() {
    // Hex-Digest from Blake2b-128bits of "TEST"
    let input_hex = "5AECF81369E312D6A3A92DD14CEDCD66";

    // Convert To Binary And Unwraps The Result | Input can be &str or String
    let bin_string = BinaryString::from_hex(input_hex).unwrap();

    // Prints Out Information
    println!("Blake2b Hexadecimal: {}", input_hex);
    println!("Binary String: {}", bin_string);
}

fn str_func() {
    // Converts &str to Binary String
    let bin_string = BinaryString::from("Hello");

    println!("Binary: {}", bin_string)
}

fn string() {
    // UTF-8 String of "Hello World" | 11 bytes
    // Then Convert To Binary From String Using A Variable And Clone (If Possible, avoid using clone for performance)
    let utf8_string = String::from("Hello World");
    let bin_string = BinaryString::from(utf8_string.clone());

    // Or, Even Easier And In One Line
    let bin_string_easy = BinaryString::from(String::from("Hello World"));

    // Asserts Both Strings Are Equal and Asserts Hello World In Binary
    assert_eq!(bin_string, bin_string_easy);
    assert_eq!(
        bin_string.0,
        "0100100001100101011011000110110001101111001000000101011101101111011100100110110001100100"
    );

    // Prints Out Information
    println!("UTF-8 String: {}", &utf8_string);
    println!("Binary String: {}", bin_string);
}

fn vector() {
    // A Vector of 4 bytes
    let input_vector: Vec<u8> = vec![111, 23, 55, 28];

    // Conversion of Vector into Binary String (can also choose to use a slice of the vector to implement copy)
    let bin_string = BinaryString::from(input_vector);

    // Prints Out Information
    println!("Vector of [111,23,55,28] u8 bytes");
    println!("Binary String {}", bin_string);
}

fn byte() {
    // Single u8 Byte
    let byte: u8 = 111u8;

    // `BinaryString` from a Single Byte
    let bin_string = BinaryString::from(byte);

    // Attempt To Add Spaces
    let spaces = bin_string.add_spaces().unwrap();

    assert_eq!(bin_string, spaces);
}
