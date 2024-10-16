use crate::prelude::*;
use std::str::FromStr;

use super::{Encrypter, ALPHABET};

#[derive(Clone, Debug, Default)]
pub struct Vigenere;

impl FromStr for Vigenere {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        if input.to_uppercase() == "VIGENERE" {
            Ok(Self)
        } else {
            Err(Error::InvalidMethodError(input.to_string()))
        }
    }
}

impl Encrypter for Vigenere {
    fn encrypt(self, context_word: &[u8], service_word: &[u8]) -> Result<Box<[u8]>> {
        // Getting the unique variable pass
        let unique = String::from_utf8(context_word.to_vec())
            .map_err(|_| Error::InvalidCharacterError)?
            .to_lowercase();

        let variable = String::from_utf8(service_word.to_vec())
            .map_err(|_| Error::InvalidCharacterError)?
            .to_lowercase();

        // Creating the new pass and initializing it empty
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
                i - (variable.len() * (i / variable.len()))
            };

            // Just an alias for the alphabet lenght as a i8
            let alphabet_len = ALPHABET.len() as i8;

            // Get the index of the current unique pass character from the alphabet.
            // If there is some character that is not in the alphabet, like a special character or number, just append
            // it to the new pass.
            let pos_u = match ALPHABET.iter().position(|&s| s == c) {
                Some(u) => u as i8,
                None => {
                    new_pass.push(c);
                    continue;
                }
            };

            // Get the index of the current variable pass character from the alphabet. If the character
            // is not on the defined alphabet it returns a error since every character here should
            // match a valid character on the unique pass.
            let pos_v: i8 = ALPHABET
                .iter()
                .position(|&s| s == variable.as_bytes()[variable_index] as char)
                .ok_or(Error::InvalidCharacterError)?
                .try_into()
                .unwrap();

            // Get the index of the new charater on the alphabet based on the unique and variable pass.
            let mut position = pos_u - alphabet_len + pos_v;
            if position < 0 {
                position += alphabet_len;
            }

            // Get the new character on the alphabet.
            let new_character = ALPHABET[position as usize];

            i += 1;
            new_pass.push(new_character);
        }

        Ok(Box::from(new_pass.as_bytes()))
    }
}
