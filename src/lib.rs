
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum SymetricMethod {
    Vigenere, B64, Xor, Enigma,
}

impl SymetricMethod {

    pub fn get_methods() -> HashMap<String, SymetricMethod> {
        let mut methods: HashMap<String, SymetricMethod> = HashMap::new();

        methods.insert(String::from("Vigenere"), SymetricMethod::Vigenere);
        methods.insert(String::from("Base64"), SymetricMethod::B64);
        methods.insert(String::from("Xor"), SymetricMethod::Xor);
        methods.insert(String::from("Enigma"), SymetricMethod::Enigma);

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

    pub fn gen_pass(method: &Self, uw: &String, vw: &String) -> String {
        match method {
            Self::Vigenere => Self::vigenere(&uw, &vw),
            Self::B64 => Self::b64(&uw),
            Self::Xor => Self::xor(&uw, &vw),
            Self::Enigma => Self::enigma(&uw, &vw),
        }
    }

}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct LoginData {
    symetric_method: SymetricMethod,
    suw: String,
    svw: String,
    pub cpw: String
}

impl LoginData {
    pub fn new (method: SymetricMethod, unique_word: String, variable_word: String) -> LoginData {
        LoginData {
            cpw: SymetricMethod::gen_pass(&method, &unique_word, &variable_word),
            symetric_method: method,
            suw: unique_word,
            svw: variable_word,
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn methods_test() {
        let uw: String = String::from("uniquepass");
        let vw: String = String::from("variablepass");

        assert_eq!(SymetricMethod::vigenere(&uw, &vw), "pnzyufaehs");
        assert_eq!(SymetricMethod::b64(&vw), "dmFyaWFibGVwYXNz");
        assert_eq!(SymetricMethod::xor(&uw, &vw), format!("{}, {}", &uw, &vw));
        assert_eq!(SymetricMethod::enigma(&uw, &vw), format!("{}, {}", &uw, &vw));
    }

    #[test]
    fn gen_test(){
        let method_enum = vec![
            SymetricMethod::Vigenere,
            SymetricMethod::B64,
            SymetricMethod::Xor,
            SymetricMethod::Enigma,
        ];
        let uw: String = String::from("uniquepass");
        let vw: String = String::from("variablepass");
        
        assert_eq!(SymetricMethod::gen_pass(&method_enum[0], &uw, &vw), "pnzyufaehs");
        assert_eq!(SymetricMethod::gen_pass(&method_enum[1], &vw, &uw), "dmFyaWFibGVwYXNz");
        assert_eq!(SymetricMethod::gen_pass(&method_enum[2], &uw, &vw), format!("{}, {}", &uw, &vw));
        assert_eq!(SymetricMethod::gen_pass(&method_enum[3], &uw, &vw), format!("{}, {}", &uw, &vw));
    }

    #[test]
    fn hashmap_test() {
        let methods: HashMap<String, SymetricMethod> = SymetricMethod::get_methods();
        assert_eq!(Some(&SymetricMethod::Vigenere), methods.get("Vigenere"));
        assert_eq!(Some(&SymetricMethod::B64), methods.get("Base64"));
        assert_eq!(Some(&SymetricMethod::Enigma), methods.get("Enigma"));
        assert_eq!(Some(&SymetricMethod::Xor), methods.get("Xor"));
    }

    #[test]
    fn login_test(){
        let method_enum = vec![
            SymetricMethod::Vigenere,
            SymetricMethod::B64,
            SymetricMethod::Xor,
            SymetricMethod::Enigma,
        ];
        let uw: String = String::from("uniquepass");
        let vw: String = String::from("variablepass");

        assert_eq!(LoginData::new(method_enum[0].clone(), "longer".to_string(), "short".to_string()), LoginData {
            symetric_method: method_enum[0].clone(),
            suw: "longer".to_string(),
            svw: "short".to_string(),
            cpw: "dvbxxj".to_string()
        });
        assert_eq!(LoginData::new(method_enum[0].clone(), uw.clone(), vw.clone()), LoginData {
            symetric_method: method_enum[0].clone(),
            suw: uw.clone(),
            svw: vw.clone(),
            cpw: "pnzyufaehs".to_string()
        });
        assert_eq!(LoginData::new(method_enum[1].clone(), vw.clone(), "".to_string()), LoginData {
            symetric_method: method_enum[1].clone(),
            suw: vw.clone(),
            svw: "".to_string(),
            cpw: "dmFyaWFibGVwYXNz".to_string()
        });
        assert_eq!(LoginData::new(method_enum[2].clone(), uw.clone(), vw.clone()), LoginData {
            symetric_method: method_enum[2].clone(),
            suw: uw.clone(),
            svw: vw.clone(),
            cpw: format!("{}, {}", &uw, &vw)
        });
        assert_eq!(LoginData::new(method_enum[3].clone(), uw.clone(), vw.clone()), LoginData {
            symetric_method: method_enum[3].clone(),
            suw: uw.clone(),
            svw: vw.clone(),
            cpw: format!("{}, {}", &uw, &vw)
        });
    }
}
