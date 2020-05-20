use to_binary::BinaryString;

#[test]
fn test_spaces() {
    // Conversion To `BinaryString` Struct From Hex
    let x: BinaryString = BinaryString::from_hex("FF8628AA").unwrap();

    // Creates New Struct With Spaces
    let with_spaces: BinaryString = x.add_spaces().unwrap();

    // Removes Spaces And Creates New Struct
    let removed_spaces: BinaryString = with_spaces.remove_spaces();

    // Prints Out Information
    println!("x: {}", x);
    println!("with_spaces : {}", with_spaces);
    println!("removed spaces: {}", removed_spaces);

    let is_true: bool = with_spaces.assert_binary_whitespace();

    // Asserts It Works And The Answer Without Spaces Is Same As Initial
    assert_eq!(is_true, true);
    assert_eq!(x, removed_spaces);
}
