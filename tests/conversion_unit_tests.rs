use to_binary::ToBinary;

#[test]
fn from_hex() {
    // "Hello" in Hexadecimal
    let bin: String = ToBinary::from_hex("48656C6C6F").unwrap();
    
    // Asserts The Binary String Is Equal To "Hello" in Hexadecimal
    assert_eq!(bin,"0100100001100101011011000110110001101111");
}

#[test]
fn from_string(){
    // "Test" as a UTF-8 String
    let bin: String = ToBinary::from_string(String::from("Test"));

    // Asserts The Binary String Is Equal To "Test" in UTF-8 As A String
    assert_eq!(bin,"01010100011001010111001101110100")
}

#[test]
fn from_string_japanese(){
    // "私はガ"
    let bin: String = ToBinary::from_string(String::from("私はガ"));

    // Asserts they are equal
    assert_eq!(bin,"111001111010011110000001111000111000000110101111111000111000001010101100")
}

#[test]
fn from_vector(){
    let mut vector: Vec<u8> = Vec::new();

    vector.push(45u8);
    vector.push(36u8);
    vector.push(117u8);

    let bin = ToBinary::from_vector(vector);

    assert_eq!(bin,"001011010010010001110101")
}

#[test]
fn from_array_slice(){
    let x: [u8;6] = [32,45,117,23,54,89];

    let bin = ToBinary::from_slice(&x);

    assert_eq!(bin,"001000000010110101110101000101110011011001011001");
}

#[test]
fn from_str(){
    let bin = ToBinary::from_str("Hello");

    assert_eq!(bin,"0100100001100101011011000110110001101111")
}