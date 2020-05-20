
/// # To-Binary: A Binary String Conversion Library
/// There are two types of Binary Strings that are present:
/// - "Binary String (0bString)"
/// - "Binary Whitespace String (0bWString)"
///
// TODO| Use ByteOrder
use hex::FromHexError;
use std::fmt;


/// # `BinaryString` Error-Handling
/// This is a enum that is used specifically for **Error-Handling**
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum BinaryError {
    InvalidCharacters,
    AlreadyHasWhitespace,
    InvalidByteSize,
    UnknownError,
}

/// # BinaryString (`0bString` + `0bWString`)
/// This Struct is the Main Struct for Dealing with **Binary Strings** and **Binary Whitespace Strings**.
///
/// It is a **Tuple Struct** that only has a **Single, Public Field** which is a `String` and can be accessed with `self.0`.
///
/// ## How To Access The `String` Field
/// ```
/// use to_binary::BinaryString;
///
/// // Generate Binary String From &str
/// let x = BinaryString::from("Hello");
///
/// // Access The Public "String" Field And Assign It To bin_string using field
/// let bin_string: String = x.0;
///
/// // Print Out Information
/// println!("{}",bin_string);
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Default)]
pub struct BinaryString(pub String);

/// # Binary Usage
/// The `BinaryUsage` Struct is used to perform functions or actions on `Binary Strings`, such as:
///
/// - Adding/Removing Whitespace
/// - Asserting Binary Strings
/// - Asserting Binary Whitespace Strings
/// - Asserting Bytes
/// - Counting Bits
/// - Counting Bytes
#[deprecated]
pub struct BinaryUsage;

// &str
impl From<&str> for BinaryString {
    fn from(n: &str) -> Self {
        let bytes = n.as_bytes();

        let mut bin_string: String = String::new();

        for byte in bytes {
            bin_string.push_str(&format!("{:08b}", byte));
        }
        return BinaryString(bin_string);
    }
}
// String
impl From<String> for BinaryString {
    fn from(n: String) -> Self {
        // Get Vector of Bytes From String
        let bytes: Vec<u8> = n.into_bytes();

        // Init a Binary String
        let mut bin_string: String = String::new();

        for byte in bytes {
            bin_string.push_str(&format!("{:08b}", byte));
        }
        return BinaryString(bin_string);
    }
}
// u8 slice
impl From<&[u8]> for BinaryString {
    fn from(byte_slice: &[u8]) -> Self {
        let bytes: Vec<u8> = byte_slice.to_vec();

        let mut bin_string: String = String::new();

        for byte in bytes {
            bin_string.push_str(&format!("{:08b}", byte));
        }
        return BinaryString(bin_string);
    }
}
// u8 array of 32
impl From<[u8; 32]> for BinaryString {
    fn from(array: [u8; 32]) -> Self {
        let mut bin_string: String = String::new();

        for byte in &array {
            bin_string.push_str(&format!("{:08b}", byte));
        }
        return BinaryString(bin_string);
    }
}
// u8 vector
impl From<Vec<u8>> for BinaryString {
    fn from(bytes: Vec<u8>) -> BinaryString {
        let mut bin_string: String = String::new();

        for byte in bytes {
            bin_string.push_str(&format!("{:08b}", byte));
        }
        return BinaryString(bin_string);
    }
}
// u8 byte
impl From<u8> for BinaryString {
    fn from(byte: u8) -> BinaryString {
        let mut bin_string: String = String::new();
        
        bin_string.push_str(&format!("{:08b}", byte));
        return BinaryString(bin_string);
    }
}

// Other Conversions
impl BinaryString {
    /// Takes as input hexadecimal and outputs a Result containing a `BinaryString` and on Error, `FromHexError`
    pub fn from_hex<T: AsRef<[u8]>>(n: T) -> Result<BinaryString, FromHexError> {
        // Decode as Hexadecimal and Unwrap
        let bytes: Vec<u8> = hex::decode(n).unwrap();

        // Init a Binary String
        let mut bin_string: String = String::new();

        for byte in bytes {
            bin_string.push_str(&format!("{:08b}", byte));
        }
        return Ok(BinaryString(bin_string));
    }
}

impl fmt::Display for BinaryString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BinaryString {
    /// Asserts The Input Is A `Binary String`, or a string with only:
    ///
    /// - `0`
    /// - `1`
    ///
    /// This function is the same as `assert_binary_string()` just with a different name
    pub fn assert_binary(&self) -> bool {
        for x in self.0.chars() {
            if x == '0' || x == '1' {
            } else {
                return false;
            }
        }
        return true;
    }
    /// Asserts the Input Is A `Binary String`, or a String with only:
    /// - `0`
    /// - `1`
    /// - No Whitespace
    pub fn assert_binary_string(&self) -> bool {
        for x in self.0.chars() {
            if x == '0' || x == '1' {
            } else {
                return false;
            }
        }
        return true;
    }
    /// Asserts The Input Is A `Binary Whitespace String`, or a String with only:
    /// - `0`
    /// - `1`
    /// - `whitespace`
    ///
    /// TODO: Add a check for whitespace to be every 8th character
    pub fn assert_binary_whitespace(&self) -> bool {
        for x in self.0.chars() {
            if x == '0' || x == '1' || x == ' ' {
            } else {
                return false;
            }
        }
        return true;
    }
    /// Asserts The Input Has (`8` * `n`) bits or contains full bytes using the remainder function
    ///
    /// This function calls the `bits()` method to retrieve the number of bits then does operation `%` 8 and compares it against 0
    pub fn assert_bytes(&self) -> bool {
        if self.bits().unwrap() % 8usize == 0usize {
            return true;
        } else {
            return false;
        }
    }
    /// Asserts The Number of Bits In The String
    ///
    /// The function `bits()` is called to compare to the parameter
    pub fn assert_bit_length(&self, len: usize) -> bool {
        if self.bits().unwrap() == len {
            return true;
        } else {
            return false;
        }
    }
    /// Asserts The Number of Bytes In The String
    ///
    /// The function `bytes()` is called to compare to the parameter
    pub fn assert_byte_length(&self, len: usize) -> bool {
        if self.bytes().unwrap() == len {
            return true;
        } else {
            return false;
        }
    }
    /// Count number of bits for both a "Binary String" and "Binary Whitespace String" and returns a Result of either a `usize` or a `BinaryError`
    pub fn bits(&self) -> Result<usize, BinaryError> {
        if self.assert_binary_string() == true {
            return Ok(self.0.len());
        } else if self.assert_binary_whitespace() == true {
            let no_spaces = self.remove_spaces();
            return Ok(no_spaces.0.len());
        } else {
            return Err(BinaryError::InvalidCharacters);
        }
    }
    /// Count number of bytes for both a "Binary String" and "Binary Whitespace String" and returns a Result of either a `usize` or on error, empty response
    pub fn bytes(&self) -> Result<usize, BinaryError> {
        if self.assert_bytes() == true {
            return Ok(self.bits().unwrap() / 8usize);
        } else if self.assert_bytes() == false {
            return Err(BinaryError::InvalidByteSize);
        } else {
            Err(BinaryError::UnknownError)
        }
    }
    /// Removes All Whitespace From Binary String And Returns A `BinaryString`
    pub fn remove_spaces(&self) -> BinaryString {
        BinaryString(self.0.chars().filter(|c| !c.is_whitespace()).collect())
    }
    /// Adds Whitespace Between Every **Byte** (8 bits)
    /// 
    /// ## Example Usage
    /// ```
    /// // Imports
    /// use to_binary::{BinaryString,BinaryError};
    /// 
    /// // Generate `BinaryString` from &str "Test String"
    /// let x = BinaryString::from("Test String");
    /// 
    /// // Returns a `BinaryString` With Spaces Between Every Byte (8 bits)
    /// let bin_with_spaces = x.add_spaces().unwrap();
    /// 
    /// ```
    pub fn add_spaces(&self) -> Result<BinaryString, BinaryError> {
        // Checks to See If Its A Binary String (0bString)
        if self.assert_binary_string() == true && self.assert_bytes() {
            // Init Binary String With Spaces
            let mut binary_string_with_spaces = self.0.clone();

            // Get Byte Count
            let bytes_to_index: usize = self.bytes().unwrap() - 1usize;

            // Start Index Counter at 8
            let mut counter: usize = 8;

            // Add Spaces At Index 8 + (9 * (bytes - 1))
            for _ in 0..bytes_to_index {
                binary_string_with_spaces.insert(counter, ' ');
                counter = counter + 9usize;
            }

            return Ok(BinaryString(binary_string_with_spaces));
        } else if self.assert_binary_whitespace() == true {
            return Err(BinaryError::AlreadyHasWhitespace);
        } else {
            Err(BinaryError::InvalidCharacters)
        }
    }
}

// Keep For Now
#[allow(deprecated)]
impl BinaryUsage {
    /// Asserts The Input Is A `Binary String`, or a string with only:
    ///
    /// - `0`
    /// - `1`
    ///
    /// This function is the same as `assert_binary_string()` just with a different name
    pub fn assert_binary(bin: String) -> bool {
        for x in bin.chars() {
            if x == '0' || x == '1' {
            } else {
                return false;
            }
        }
        return true;
    }
    /// Asserts the Input Is A `Binary String`, or a String with only:
    /// - `0`
    /// - `1`
    /// - No Whitespace
    pub fn assert_binary_string(bin: String) -> bool {
        for x in bin.chars() {
            if x == '0' || x == '1' {
            } else {
                return false;
            }
        }
        return true;
    }
    /// Asserts The Input Is A `Binary Whitespace String`, or a String with only:
    /// - `0`
    /// - `1`
    /// - `whitespace`
    ///
    /// TODO: Add a check for whitespace to be every 9nth character
    pub fn assert_binary_whitespace(bin: String) -> bool {
        for x in bin.chars() {
            if x == '0' || x == '1' || x == ' ' {
            } else {
                return false;
            }
        }
        return true;
    }
    /// Asserts The Input Has (`8` * `n`) bits or contains full bytes using the remainder function
    pub fn assert_bytes(bin: String) -> bool {
        if BinaryUsage::count_bits(bin).unwrap() % 8usize == 0usize {
            return true;
        } else {
            return false;
        }
    }
    /// Count number of bits for both a "Binary String" and "Binary Whitespace String" and returns a Result of either a `usize` or on error, empty response
    pub fn count_bits(bin: String) -> Result<usize, ()> {
        if BinaryUsage::assert_binary_string(bin.clone()) == true {
            return Ok(bin.len());
        } else if BinaryUsage::assert_binary_whitespace(bin.clone()) == true {
            return Ok(BinaryUsage::remove_spaces(bin).len());
        } else {
            return Err(());
        }
    }
    /// Count number of bytes for both a "Binary String" and "Binary Whitespace String" and returns a Result of either a `usize` or on error, empty response
    pub fn count_bytes(bin: String) -> Result<usize, ()> {
        if BinaryUsage::assert_bytes(bin.clone()) == true {
            return Ok(BinaryUsage::count_bits(bin).unwrap() / 8usize);
        } else {
            Err(())
        }
    }
    pub fn add_spaces(bin: String) -> Result<String, ()> {
        // Checks to See If Its A Binary String (0bString)
        if BinaryUsage::assert_binary_string(bin.clone()) == true
            && BinaryUsage::assert_bytes(bin.clone())
        {
            // Init Binary String With Spaces
            let mut binary_string_with_spaces = bin.clone();

            // Get Byte Count
            let bytes_to_index: usize = BinaryUsage::count_bytes(bin).unwrap() - 1usize;

            // Start Index Counter
            let mut counter: usize = 8;

            // Add Spaces At Index 8 + (9 * (bytes - 1))
            for _ in 0..bytes_to_index {
                binary_string_with_spaces.insert(counter, ' ');
                counter = counter + 9usize;
            }
            return Ok(binary_string_with_spaces);
        } else if BinaryUsage::assert_binary_whitespace(bin.clone()) == true {
            return Ok(bin);
        } else {
            Err(())
        }
    }
    /// Removes All Whitespace From String And Returns String
    pub fn remove_spaces(bin: String) -> String {
        bin.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
