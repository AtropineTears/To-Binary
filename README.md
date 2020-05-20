# To-Binary: A Rust Crate For Conversion To A `BinaryString`
![Crates.io](https://img.shields.io/crates/v/to-binary?style=flat-square)![Crates.io](https://img.shields.io/crates/l/to-binary?style=flat-square)

A Rust Library For Conversion To new type `BinaryString`. `BinaryString` is a singled-fielded, tuple struct that holds a **BinaryString** or **BinaryWhitespaceString** and is of the type `String`.

It allows: 

* Asserting Whether a Given Input Is a Binary String
* Asserting Number of Bits/Bytes
* Counting of Bits and Bytes
* Adding/Removing Whitespace Between Bytes

## How To Use

Read **Examples** and **Read The Documentation**

### Conversion To `BinaryString`

These examples show how you can convert from one type to the `BinaryString` type.

```rust
use to_binary::{BinaryString,BinaryError};

fn main(){
    // Hexadecimal
    let hex = BinaryString::from_hex("2879E653864EA6047FEBBBD9AE6DA8DA").unwrap();
    
    // String
    let x = BinaryString::from(String::from("Test"));
    assert_eq!(bin_string_2,"01010100011001010111001101110100")
  
  	// str
  	let y = BinaryString::from("Hello World");
  
  	// Byte
  	let byte = BinaryString::from(118u8);
  	
  	// Vector
  	let vector = vec![36,57,123,38,2];
  	let bin_vector = BinaryString::from(vector);
}
```

### Adding or Removing Whitespace From A `BinaryString`

This example shows how you can add whitespace between bytes or remove whitespace from a `BinaryString`

```rust
use to_binary::{BinaryString,BinaryError};

fn test_whitespace() {
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
```

### Using Binary Methods On `BinaryString`

There are many `BinaryString` Methods That Can Be Used. This is a few of them that may be useful.

```rust
fn main(){
  	// Conversion To `BinaryString` Struct From Hex
  	let x: BinaryString = BinaryString::from_hex("FF8628AA").unwrap();
  
  	// Checks Whether The Input Is Binary
    let check_if_binary: bool = x.assert_binary();

    // Assert Input Is Binary
    assert_eq!(check_if_binary, true);

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
```

## License

Licensed under either of

* Apache License, Version 2.0

* MIT license

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.