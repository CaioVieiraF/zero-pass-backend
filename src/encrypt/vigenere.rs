use crate::{CipherError, CipherResult, Method};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Vigenere;

impl Method for Vigenere {
    fn encrypt(&self, uw: impl Into<String>, vw: impl Into<String>) -> CipherResult {
        // Definition of the alphabet used
        let alphabet = "abcdefghijklmnopqrstuvwxyz";

        // Getting the unique variable pass
        let unique: String = uw.into().to_lowercase();
        let variable: String = vw.into().to_lowercase();

        // Creating the new pass and initializing it empity
        let mut new_pass = String::new();

        // Counter to control valid characters on the alphabet
        let mut i = 0;

        // Loop to set the new characters to the new password
        for c in unique.chars() {
            // Get the index of the current character on variable pass.
            // This formula is to get the value even when the unique pass length is larger than variable
            // pass length.
            let variable_index = if i < variable.len() {
                i
            } else {
                i - (variable.len() * (i / variable.len() as usize))
            };

            // Just an alias for the alphabet lenght as a i8
            let alphabet_len = alphabet.len() as i8;

            // Get the index of the current unique pass character from the alphabet.
            // If there is something that is not there, like a special character or number, just append
            // it to the new pass.
            let pos_u = match alphabet.find(&c.to_string()) {
                Some(u) => u as i8,
                None => {
                    new_pass += &c.to_string();
                    continue;
                }
            };

            // Get the index of the current variable pass character from the alphabet. If the character
            // is not on the defined alphabet it returns a error since every character here should
            // match a valid character on the unique pass.
            let pos_v = match alphabet.find(variable.as_bytes()[variable_index] as char) {
                Some(v) => v as i8,
                None => return Err(CipherError::InvalidCharacterError),
            };
            drop(variable_index);

            // Get the index of the new charater on the alphabet based on the unique and variable pass.
            let mut position = pos_u - alphabet_len + pos_v;
            if position < 0 {
                position += alphabet_len;
            }

            // Get the new character on the alphabet.
            let new_character = alphabet.as_bytes()[position as usize] as char;

            i += 1;
            new_pass.push(new_character);
        }

        Ok(new_pass)
    }
}
