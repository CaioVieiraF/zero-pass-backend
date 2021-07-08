
use crate::Methods;

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

    for i in vw.chars() {
        if !alphabet.contains(i){
            panic!("Caracteres especiais não são permitidos!");
        }
    }

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

pub fn enigma(mut uw: Vec<String>, vw: &String, swb: &String) -> String {
    
    let variable: String = vw.to_uppercase();
    let switchboard: String = swb.to_uppercase();

    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut new_pass = String::new();
    
    for mut each in variable.chars(){
        let char_position = alphabet.find(each).expect("Only letters are permited!");
        each = switchboard.as_bytes()[char_position] as char;

        let mut scrambled = String::from(alphabet);
        for rotor in &uw{
            let char_position = scrambled.find(each).expect("Unexpected error!");
            each = rotor.as_bytes()[char_position] as char;
            scrambled = rotor.to_string();
        }

        let char_position = alphabet.find(each).expect("Unexpected error!");
        each = alphabet.as_bytes()[alphabet.len() - 1 - char_position] as char;
        
        uw.reverse();
        let mut scrambled = &uw[0];
        for rotor in &uw[1..]{
            let char_position = scrambled.find(each).expect("Unexpected error!");
            each = rotor.as_bytes()[char_position] as char;
            scrambled = rotor;
        }
        let char_position = scrambled.find(each).expect("Unexpected error!");
        each = alphabet.as_bytes()[char_position] as char;



        let char_position = switchboard.find(each).expect("Unexpected error!");
        each = alphabet.as_bytes()[char_position] as char;

        new_pass.push(each);
    }

    new_pass
}

pub fn gen_pass(method: &Methods) -> String {

    match method {
        Methods::Vigenere { word, password } => vigenere(word, password),
        Methods::B64 { word } => b64(word),
        Methods::Xor { word, password } => xor(word, password),
        Methods::Enigma { message, switchboard, rotors } => {
            enigma(rotors.to_vec(), message, switchboard)
        },
    }
}

