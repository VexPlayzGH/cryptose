pub use self::encryption::encrypt;
pub use self::decryption::decrypt;
/// Encrypts a string into a string consisting of ASCII u8 integers.
/// 
/// # Examples:
/// 
/// ```
/// let arg = String::from("hello");
/// let encrypted = cryptose::encrypt(&arg);
/// 
/// assert_eq!("730173317311173801738017310173401".to_string(), encrypted.trim());
/// ```
pub mod encryption {
    pub fn encrypt(strg: &String) -> String {
        let mut vec: Vec<char> = Vec::new();
        for l in strg.chars() {
            vec.push(l);
        }
        let map_enc = vec.iter().map(|e| e.to_ascii_lowercase() as u8);
        let vec_enc: Vec<u8> = map_enc.clone().collect();
        let mut str_enc: String = String::new();
        for e in vec_enc.iter() {
            str_enc.push_str(e.to_string().as_str());
            str_enc.push_str("37");
        }
        let str_final = str_enc.trim().chars().rev().collect::<String>();
        str_final
    }
}
/// Decrypts an encrypted - via the cryptose::encrypt() function - string,
/// consisting of ASCII u8 integers into a string.
/// 
/// # Examples:
/// 
/// ```
/// let arg = String::from("730173317310173997350173011");
/// let decrypted = cryptose::decrypt(&arg);
/// 
/// assert_eq!("nice".to_string(), decrypted.trim());
/// ```
/// 
/// # Panics (AVOID doing these!):
/// 
/// -- Calling the decrypt() function with a string of non-integer characters as a parameter.
/// 
/// ```
/// let arg_panics = String::from("hello");
/// let decrypted = cryptose::decrypt(&arg_panics);
/// ```
/// 
/// The above block of code will cause your program to panic.
pub mod decryption {
    pub fn decrypt(strg: &String) -> String {
        let str_enc = strg.trim().chars().rev().collect::<String>();
        let str1 = str_enc.replace("37", " ");
        let vec1: Vec<&str> = str1.split_whitespace().collect();
        let map = vec1.iter().map(|_e| _e.parse::<u8>().unwrap() as char);
        let vec2: Vec<char> = map.clone().collect::<Vec<char>>();
        let mut str_dec: String = String::new();
        for e in vec2.iter() {
            str_dec.push_str(e.to_string().as_str());
        }
        str_dec
    }
}