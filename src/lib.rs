/// # To-Binary: A Binary String Conversion Library
/// There are two types of Binary Strings that are present:
/// - "Binary String (0bString)"
/// - "Binary Whitespace String (0bWString)"


use hex::FromHexError;

/// # Conversion To Binary
/// The `ToBinary` Struct is used to convert from one data type to `binary strings`, or a string that only contains `0` and `1`
pub struct ToBinary;
pub struct FromBinary;

/// # Binary Usage
/// The `BinaryUsage` Struct is used to perform functions or actions on `Binary Strings`, such as:
/// 
/// - Adding/Removing Whitespace
/// - Asserting Binary Strings
/// - Asserting Binary Whitespace Strings
/// - Asserting Bytes
/// - Counting Bits
/// - Counting Bytes
pub struct BinaryUsage;

impl ToBinary {
    pub fn from_slice(byte_slice: &[u8]) -> String {
        let bytes: Vec<u8> = byte_slice.to_vec();
        
        let mut bin_string: String = String::new();

         for byte in bytes {
            bin_string.push_str(&format!("{:08b}",byte));
        }
        return bin_string
        
    }
    pub fn from_vector(bytes: Vec<u8>) -> String {
        let mut bin_string: String = String::new();

         for byte in bytes {
            bin_string.push_str(&format!("{:08b}",byte));
        }
        return bin_string
    }
    pub fn from_string(n: String) -> String {
        // Get Vector of Bytes From String
        let bytes: Vec<u8> = n.into_bytes();

        // Init a Binary String
        let mut bin_string: String = String::new();

         for byte in bytes {
            bin_string.push_str(&format!("{:08b}",byte));
        }
        return bin_string
    }
    pub fn from_hex<T: AsRef<str>>(n: T) -> Result<String,FromHexError> {
        // Decode as Hexadecimal and Unwrap
        let bytes: Vec<u8> = hex::decode(n.as_ref()).unwrap();
        
        // Init a Binary String
        let mut bin_string: String = String::new();
        
        
        for byte in bytes {
            bin_string.push_str(&format!("{:08b}",byte));
        }
        return Ok(bin_string)
    }
    pub fn from_str(n: &str) -> String {
        let bytes = n.as_bytes();

        let mut bin_string: String = String::new();

        for byte in bytes {
            bin_string.push_str(&format!("{:08b}",byte));
        }
        return bin_string
    }
}

impl BinaryUsage {
    /// Asserts The Input Is A `Binary String`, or a string with only:
    /// - `0`
    /// - `1`
    /// 
    /// This function is the same as `assert_binary_string()` just with a different name
    pub fn assert_binary(bin: String) -> bool {
        for x in bin.chars() {
            if x == '0' || x == '1' {

            }
            else {
                return false
            }
        }
        return true
    }
    /// Asserts the Input Is A `Binary String`, or a String with only: 
    /// - `0`
    /// - `1`
    /// - No Whitespace
    pub fn assert_binary_string(bin: String) -> bool {
        for x in bin.chars() {
            if x == '0' || x == '1' {

            }
            else {
                return false
            }
        }
        return true
    }
    /// Asserts The Input Is A `Binary Whitespace String`, or a String with only: 
    /// - `0`
    /// - `1`
    /// - `whitespace`
    /// TODO: Add a check for whitespace to be every 9nth character
    pub fn assert_binary_whitespace(bin: String) -> bool {
        for x in bin.chars() {
            if x == '0' || x == '1' || x == ' ' {
                
            }
            else {
                return false
            }
        }
        return true
    }
    /// Asserts The Input Has (`8` * `n`) bits or contains full bytes using the remainder function
    pub fn assert_bytes(bin: String) -> bool {
        if BinaryUsage::count_bits(bin).unwrap() % 8usize == 0usize {
            return true
        }
        else {
            return false
        }
    }
    /// Count number of bits for both a "Binary String" and "Binary Whitespace String" and returns a Result of either a `usize` or on error, empty response
    pub fn count_bits(bin: String) -> Result<usize,()> {
        if BinaryUsage::assert_binary_string(bin.clone()) == true {
            return Ok(bin.len())
        }
        else if BinaryUsage::assert_binary_whitespace(bin.clone()) == true {
            return Ok(BinaryUsage::remove_spaces(bin).len())
        }
        else {
            return Err(())
        }
    }
    /// Count number of bytes for both a "Binary String" and "Binary Whitespace String" and returns a Result of either a `usize` or on error, empty response
    pub fn count_bytes(bin: String) -> Result<usize,()> {
        if BinaryUsage::assert_bytes(bin.clone()) == true {
            return Ok(BinaryUsage::count_bits(bin).unwrap() / 8usize)
        }
        else {
            Err(())
        }
    }
    pub fn add_spaces(bin: String) -> Result<String,()> {
        // Checks to See If Its A Binary String (0bString)
        if BinaryUsage::assert_binary_string(bin.clone()) == true && BinaryUsage::assert_bytes(bin.clone()) {
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
            return Ok(binary_string_with_spaces)
        }
        else if BinaryUsage::assert_binary_whitespace(bin.clone()) == true {
            return Ok(bin)
        }
        else {
            Err(())
        }
    }
    /// Removes All Whitespace From String And Returns String
    pub fn remove_spaces(bin: String) -> String {
        bin.chars().filter(|c| !c.is_whitespace()).collect()
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
