extern crate kennitala;

use kennitala::is_valid;

#[test]
fn valid_kennitala(){
	assert_eq!(is_valid("091280-5079"), true);
    assert_eq!(is_valid("0902862349"), true);
    assert_eq!(is_valid("140543-3229"), true);
    
}
#[test]
fn invalid_kennitala(){
	assert_eq!(is_valid("091280-079"), false);
    assert_eq!(is_valid("091280-B079"), false);
}
