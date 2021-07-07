pub mod encrypt;
pub mod login_data;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Methods {
    Vigenere, B64, Xor, Enigma,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct LoginData {
    symetric_method: Methods,
    suw: String,
    svw: String,
    pub cpw: String
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn methods_test() {
        let uw: String = String::from("uniquepass");
        let vw: String = String::from("variablepass");

        assert_eq!(encrypt::vigenere(&uw, &vw), "pnzyufaehs");
        assert_eq!(encrypt::b64(&vw), "dmFyaWFibGVwYXNz");
        assert_eq!(encrypt::xor(&uw, &vw), format!("{}, {}", &uw, &vw));
        assert_eq!(encrypt::enigma(&uw, &vw), format!("{}, {}", &uw, &vw));
    }

    #[test]
    fn gen_test(){
        let method_enum = vec![
            Methods::Vigenere,
            Methods::B64,
            Methods::Xor,
            Methods::Enigma,
        ];
        let uw: String = String::from("uniquepass");
        let vw: String = String::from("variablepass");
        
        assert_eq!(encrypt::gen_pass(&method_enum[0], &uw, &vw), "pnzyufaehs");
        assert_eq!(encrypt::gen_pass(&method_enum[1], &vw, &uw), "dmFyaWFibGVwYXNz");
        assert_eq!(encrypt::gen_pass(&method_enum[2], &uw, &vw), format!("{}, {}", &uw, &vw));
        assert_eq!(encrypt::gen_pass(&method_enum[3], &uw, &vw), format!("{}, {}", &uw, &vw));
    }

    #[test]
    fn hashmap_test() {
        use std::collections::HashMap;

        let methods: HashMap<String, Methods> = encrypt::get_methods();
        assert_eq!(Some(&Methods::Vigenere), methods.get("Vigenere"));
        assert_eq!(Some(&Methods::B64), methods.get("Base64"));
        assert_eq!(Some(&Methods::Enigma), methods.get("Enigma"));
        assert_eq!(Some(&Methods::Xor), methods.get("Xor"));
    }

}
