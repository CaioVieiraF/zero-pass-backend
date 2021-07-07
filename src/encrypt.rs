use std::collections::HashMap;
use crate::Methods;

pub fn get_methods() -> HashMap<String, Methods> {

    let mut methods: HashMap<String, Methods> = HashMap::new();

    methods.insert(String::from("Vigenere"), Methods::Vigenere);
    methods.insert(String::from("Base64"), Methods::B64);
    methods.insert(String::from("Xor"), Methods::Xor);
    methods.insert(String::from("Enigma"), Methods::Enigma);

    methods
}

pub fn vigenere(uw: &String, vw: &String) -> String{
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
                    position += alphabet.find(variable.as_bytes()[{
                        if x > variable.len() - 1 {
                            x - (variable.len() * (x/variable.len() as usize))
                        } else {
                            x
                        }
                    }] as char).unwrap();
                    position %= alphabet.len();

                    x += 1;
                    alphabet.as_bytes()[position] as char
                },
                None => i,
            }
        );
    }

    new_pass

}

pub fn b64(vw: &String) -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut binary_word = "".to_string();
    let mut new_pass = String::new();

    for i in vw.clone().into_bytes() {
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
    new_pass
}

pub fn xor(uw: &String, vw: &String) -> String {
    String::from(format!("{}, {}", uw, vw))
}

pub fn enigma(uw: &String, vw: &String) -> String {
    String::from(format!("{}, {}", uw, vw))
}

pub fn gen_pass(method: &Methods, uw: &String, vw: &String) -> String {
    match method {
        Methods::Vigenere => vigenere(&uw, &vw),
        Methods::B64 => b64(&uw),
        Methods::Xor => xor(&uw, &vw),
        Methods::Enigma => enigma(&uw, &vw),
    }
}

