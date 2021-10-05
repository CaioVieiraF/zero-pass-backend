use crate::{CipherResult, CipherError};

pub fn vigenere(uw: &str, vw: &str) -> CipherResult {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let unique: String = uw.to_lowercase();
    let variable: String = vw.to_lowercase();
    let mut new_pass = String::new();

    let mut x = 0;

    for i in unique.chars() {
        let char_position = alphabet.find(i);
        new_pass.push(match char_position {
            Some(mut position) => {
                let character_to_find = {
                    variable.as_bytes()[{
                        if x > variable.len() - 1 {
                            x - (variable.len() * (x / variable.len() as usize))
                        } else {
                            x
                        }
                    }] as char
                };

                position += match alphabet.find(character_to_find) {
                    Some(x) => x,
                    None => return Err(CipherError::InvalidCharacterError),
                };
                position %= alphabet.len();

                x += 1;
                alphabet.as_bytes()[position] as char
            }
            None => i,
        });
    }

    Ok(new_pass)
}
