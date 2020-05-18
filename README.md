# To-Binary
A Rust Library For Conversion To A **Binary String** Supporting:

* Strings
* str
* Hexadecimal Strings
* Vector of Bytes
* Slice of Bytes

It also allows: 

* Counting of Bits and Bytes
* Asserting Whether a Given Input is a Binary String
* Removing/Adding Whitespace Between Bytes

## How To Use

Read **Examples** and read documentation

### FromBinary

```rust
use to_binary::{ToBinary,FromBinary};

fn main(){
    // Hexadecimal
    let bin_string = ToBinary::from_hex("FFAA10FF").unwrap();
    assert_eq!(bin_string,"11111111101010100001000011111111");
    
    // UTF-8
    let x = ToBinary::from_string(String::from("Test"));
    assert_eq!(bin_string_2,"01010100011001010111001101110100")
}
```

## License

Licensed under either of

* Apache License, Version 2.0

* MIT license

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.