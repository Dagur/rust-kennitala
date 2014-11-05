#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub fn is_valid(kennitala : &str) -> bool {
    let constants = [3u,2u,7u,6u,5u,4u,3u,2u];
    let re = regex!(r"^\d{6}-?\d{4}$");
    
    if re.is_match(kennitala) {
        let sanitized = kennitala.replace("-","");
        let check_digit = sanitized
            .as_slice()
            .char_at(8)
            .to_digit(10)
            .unwrap();
       
        let c = constants
            .iter()
            .zip(sanitized.as_slice().chars())
            .fold(0u, 
                |sum : uint, (x, y) : (&uint, char)| 
                sum + x * y.to_digit(10).unwrap()
            );
        
        if 11 - (c % 11) == check_digit {
            return true
        } 
    } 
    false    
}
