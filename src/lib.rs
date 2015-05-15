extern crate regex;

macro_rules! regex(
    ($s:expr) => (regex::Regex::new($s).unwrap());
);

pub fn is_valid(kennitala : &str) -> bool {
    let constants = [3, 2, 7, 6, 5, 4, 3, 2];
    let re = regex!(r"^\d{6}-?\d{4}$");
    
    if re.is_match(kennitala) {
        let sanitized = kennitala.replace("-","");
        let check_digit = (sanitized.as_bytes()[8] as char).to_digit(10).unwrap();
       
        let c = constants
            .iter()
            .zip(sanitized.bytes())
            .fold(0, 
                |sum : u32, (x, y) : (&u32, u8)| 
                sum + x * (y as char).to_digit(10).unwrap()
            );
        
        
        println!("check digit: {0}", check_digit);
        
        if 11 - (c % 11) == check_digit {
            return true
        } 
    } 
    false    
}
