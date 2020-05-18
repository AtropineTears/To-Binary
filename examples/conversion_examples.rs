use to_binary::ToBinary;

fn main(){
    hexadecimal();
    str_func();
    string();
    vector();
    slice();
}

fn hexadecimal() {
    // Hex-Digest from Blake2b-128bits of "TEST"
    let input_hex = "5AECF81369E312D6A3A92DD14CEDCD66";
    
    // Convert To Binary And Unwraps The Result | Input can be &str or String
    let bin_string = ToBinary::from_hex(input_hex).unwrap();

    // Prints Out Information
    println!("Blake2b Hexadecimal: {}",input_hex);
    println!("Binary String: {}",bin_string);
}

fn str_func(){
    // Converts &str to Binary String
    let bin_string = ToBinary::from_str("Hello");

    println!("Binary: {}",bin_string)
}

fn string(){
    // UTF-8 String of "Hello World" | 11 bytes
    // Then Convert To Binary From String Using A Variable And Clone (If Possible, avoid using clone for performance)
    let utf8_string = String::from("Hello World");
    let bin_string = ToBinary::from_string(utf8_string.clone());

    // Or, Even Easier And In One Line
    let bin_string_easy = ToBinary::from_string(String::from("Hello World"));

    // Asserts Both Strings Are Equal and Asserts Hello World In Binary
    assert_eq!(bin_string,bin_string_easy);
    assert_eq!(bin_string,"0100100001100101011011000110110001101111001000000101011101101111011100100110110001100100");

    // Prints Out Information
    println!("UTF-8 String: {}",&utf8_string);
    println!("Binary String: {}",bin_string);
}

fn vector(){
    // A Vector of 4 bytes
    let input_vector: Vec<u8> = vec![111,23,55,28];

    // Conversion of Vector into Binary String (can also choose to use a slice of the vector to implement copy)
    let bin_string = ToBinary::from_vector(input_vector);

    // Prints Out Information
    println!("Vector of [111,23,55,28] u8 bytes");
    println!("Binary String {}",bin_string);
}

fn slice(){
    // An Array of 8 bytes all 0xFF | Arrays up to 32 can implement copy trait
    let input_array: [u8;8] = [255u8;8];

    // Conversion of Slice (or slice of Array/Vector) into Binary
    let bin_string = ToBinary::from_slice(&input_array);

    // Prints Out Information
    println!("Debug: {:?}",input_array);
    println!("Binary String: {}",bin_string)
}