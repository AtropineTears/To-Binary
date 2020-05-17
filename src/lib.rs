use hex::FromHexError;

pub struct ToBinary;
pub struct FromBinary;

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
    pub fn from_utf8(n: String) -> String {
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
}
impl FromBinary {
    pub fn count_bits(bin: String) -> Result<usize,()> {
        if FromBinary::is_binary(bin.clone()) == true {
            return Ok(bin.len())
        }
        else {
            return Err(())
        }
    }
    pub fn is_binary(bin: String) -> bool {
        for x in bin.chars() {
            if x == '0' || x == '1' {

            }
            else {
                return false
            }
        }
        return true
    }
}

#[test]
fn test_binary(){
    let x = ToBinary::from_hex("FFAA10FF").unwrap();
    println!("{}",x);
    assert_eq!(x,"11111111101010100001000011111111");
}

#[test]
fn test_bin2(){
    let x = ToBinary::from_utf8(String::from("Test"));
    println!("{}",x);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
