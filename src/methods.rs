#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Methods {
    Vigenere {
        word: String,
        password: String,
    },
    B64 { word: String },
    Xor {
        word: String,
        password: String,
    },
    Enigma {
        message: String,
        switchboard: String,
        rotors: Vec<String>,
    }
}