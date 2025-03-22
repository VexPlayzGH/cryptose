use std::process::Command;
use cryptose::{encrypt, decrypt};
fn main() {
    println!("Welcome to Cryptose, a crate where you can encrypt or decrypt your desired messages with ease!");
    let mut strg: String = String::new();
    let mut exit: bool = false;
    let mut temp_encrypted: String = String::new();     
    while exit == false {            
        println!("\nPlease enter the message you wish to encrypt/decrypt.");
        std::io::stdin().read_line(&mut strg).expect("Invalid input");
        println!("\nDo you wish to encrypt the message? (y/N)");
        let mut sel0: String = String::new();
        std::io::stdin().read_line(&mut sel0).expect("Invalid input");
        match sel0.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                let encrypted = encrypt(&strg);
                temp_encrypted = encrypted;
                println!("\nEncrypted message: {}", temp_encrypted.trim());
            },
            _ => {},
        }
        println!("\nDo you wish to decrypt the message? (y/N)");
        let mut sel1: String = String::new();
        std::io::stdin().read_line(&mut sel1).expect("Invalid input");
        match sel1.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                if temp_encrypted.is_empty() {
                    let decrypted = decrypt(&strg);
                    println!("\nDecrypted message: {}", decrypted.trim());
                }
                else {
                    let decrypted = decrypt(&temp_encrypted);
                    println!("\nDecrypted message: {}", decrypted.trim());
                }
            },
            _ => {},
        }
        println!("\nDo you wish to encrypt/decrypt another message? (y/N)");
        let mut sel2: String = String::new();
        std::io::stdin().read_line(&mut sel2).expect("Invalid input");
        match sel2.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                strg.clear();
            }
            _ => exit = true,
        }
        temp_encrypted.clear();
    }
    println!("\nThank you for using Cryptose!");
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}