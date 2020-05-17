# To-Binary
A Rust Library For Conversion To A **Binary String** Supporting:

* UTF-8 Strings
* Hexadecimal Strings
* Vector of Bytes
* Slice of Bytes

It also allows the counting of bits and asserting whether a given input is a binary string.

## How To Use

### FromBinary

```rust
use to_binary::{ToBinary,FromBinary};

fn main(){
    // Hexadecimal
    let bin_string = ToBinary::from_hex("FFAA10FF").unwrap();
    assert_eq!(bin_string,"11111111101010100001000011111111");
    
    // UTF-8
    let x = ToBinary::from_utf8(String::from("Test"));
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