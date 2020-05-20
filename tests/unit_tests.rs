#[cfg(test)]
mod conversion {
    use to_binary::BinaryString;
    #[test]
    fn from_hex() {
        // "Hello" in Hexadecimal
        let bin = BinaryString::from_hex("48656C6C6F").unwrap();

        // Asserts The Binary String Is Equal To "Hello" in Hexadecimal
        assert_eq!(bin.0, "0100100001100101011011000110110001101111");
    }
    #[test]
    #[should_panic]
    fn from_hex_4_bits() {
        // "Hello" in Hexadecimal
        let _bin = BinaryString::from_hex("48656C6C6FD").unwrap();
    }

    #[test]
    fn from_string() {
        // "Test" as a UTF-8 String
        let bin = BinaryString::from(String::from("Test"));

        // Asserts The Binary String Is Equal To "Test" in UTF-8 As A String
        assert_eq!(bin.0, "01010100011001010111001101110100");
    }

    #[test]
    fn from_string_japanese() {
        // "私はガ"
        let bin = BinaryString::from(String::from("私はガ"));

        // Asserts they are equal
        assert_eq!(
            bin.0,
            "111001111010011110000001111000111000000110101111111000111000001010101100"
        )
    }

    #[test]
    fn from_vector() {
        let mut vector: Vec<u8> = Vec::new();

        vector.push(45u8);
        vector.push(36u8);
        vector.push(117u8);

        let bin = BinaryString::from(vector);

        assert_eq!(bin.0, "001011010010010001110101")
    }
    #[test]
    fn from_str() {
        let bin = BinaryString::from("Hello");

        assert_eq!(bin.0, "0100100001100101011011000110110001101111")
    }
}
#[cfg(test)]
mod advanced_tests {
    #[allow(unused_imports)]
    use to_binary::{BinaryString,BinaryError};
    #[test]
    fn single_byte_test_with_spaces() {
        // Single u8 Byte
        let byte: u8 = 111u8;
    
        // `BinaryString` from a Single Byte
        let bin_string = BinaryString::from(byte);
    
        // Attempt To Add Spaces
        let spaces = bin_string.add_spaces().unwrap();

        // Removes Spaces
        let removed_spaces = spaces.remove_spaces();
    
        // Assert Both Strings Are Equal
        assert_eq!(bin_string, spaces);
        assert_eq!(bin_string,removed_spaces);
        assert_eq!(spaces,removed_spaces);
    }
}