use to_binary::{ToBinary,BinaryUsage};

#[test]
fn test_spaces(){
    let x = ToBinary::from_hex("FFFFFFAA").unwrap();
    let with_spaces = BinaryUsage::add_spaces(x).unwrap();
    let is_true = BinaryUsage::assert_binary_whitespace(with_spaces);

    assert_eq!(is_true,true);
}