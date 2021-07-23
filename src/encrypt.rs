use crate::{ CipherError, CipherResult };

#[derive(Debug, Clone, PartialEq)]
pub struct MethodArgs<'a>{
    pub word: &'a str,
    pub password: &'a str
}

#[derive(Debug, Clone, PartialEq)]
pub enum Methods<'a>{
    Vigenere(MethodArgs<'a>), B64(MethodArgs<'a>), Xor(MethodArgs<'a>)
}

pub fn vigenere(uw: &str, vw: &str) -> CipherResult {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let unique: String = uw.to_lowercase();
    let variable: String = vw.to_lowercase();
    let mut new_pass = String::new();

    let mut x = 0;
    
    for i in unique.chars() {
        let char_position = alphabet.find(i);
        new_pass.push(
            match char_position {
                Some(mut position) => {
                    let character_to_find = {
                        variable.as_bytes()[{
                            if x > variable.len() - 1 {
                                x - (variable.len() * (x/variable.len() as usize))
                            } else {
                                x
                            }
                        }] as char
                    };

                    position += match alphabet.find(character_to_find) {
                        Some(x) => x,
                        None => return Err(CipherError::InvalidCharacterError)
                    };
                    position %= alphabet.len();

                    x += 1;
                    alphabet.as_bytes()[position] as char
                },
                None => i,
            }
        );
    }

    Ok(new_pass)

}

pub fn b64(vw: &str) -> CipherResult {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut binary_word = "".to_string();
    let mut new_pass = String::new();

    for i in vw.chars() {
        if !alphabet.contains(i){
            return Err(CipherError::InvalidCharacterError)
        }
    }

    for i in vw.clone().bytes() {
        binary_word += &format!("0{:b}", i);
    }

    let mut padding = "".to_string();
    while binary_word.len() % 6 != 0 {
        binary_word += "00";
        padding += "=";
    }

    let mut x = 0;
    let mut new_binary = String::new();
    for i in binary_word.chars(){
        if x == 6{ new_binary += " "; x = 0 }
        new_binary += &i.to_string();
        x += 1;
    }
        
    let binary_vec: Vec<&str> = new_binary.split(" ").collect();
    for i in binary_vec{
        let number = usize::from_str_radix(i, 2).unwrap();
        new_pass += &(alphabet.as_bytes()[number] as char).to_string();
    }

    new_pass += &padding;
    Ok(new_pass)
}

pub fn xor(uw: &str, vw: &str) -> CipherResult {
    Ok(String::new())
}

pub fn gen_pass(method: &Methods) -> CipherResult {

    match method {
        Methods::Vigenere(args) => vigenere(args.word, args.password),
        Methods::B64(args) => b64(args.password),
        Methods::Xor(args) => xor(args.word, args.password),
    }
}

