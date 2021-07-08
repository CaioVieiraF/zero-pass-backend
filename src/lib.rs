use crate::methods::Methods;

pub mod encrypt;
pub mod login_data;
pub mod methods;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct LoginData {
    symetric_method: Methods,
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
        assert_eq!(encrypt::enigma(
            vec![
                "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string(),
                "AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string(),
                "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string(),
                "ESOVPZJAYQUIRHXLNFTGKDCMWB".to_string(),
                "VZBRGITYUPSDNHLXAWMJQOFECK".to_string()
        ],&vw, &String::from("ATBSDEFMIRKNLZOWPVXY")), format!("{}, {}", &uw, &vw));
    }

    #[test]
    fn gen_test(){
         let vige_method = Methods::Vigenere {
        word: String::from("uniquepass"),
        password: String::from("variablepass"),
    };

    let base_method = Methods::B64 { word: String::from("variablepass") };

    let _xor_method = Methods::Xor {
        word: String::from("uniquepass"),
        password: String::from("variablepass"),
    };

    let enig_method = Methods::Enigma {
        message: String::from("variablepass"),
        switchboard: String::from("ATBSDEFMIRKNLZOWPVXY"),
        rotors: vec![
            "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string(),
            "AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string(),
            "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string(),
            "ESOVPZJAYQUIRHXLNFTGKDCMWB".to_string(),
            "VZBRGITYUPSDNHLXAWMJQOFECK".to_string()
        ]
    };

    assert_eq!(encrypt::gen_pass(&vige_method), "pnzyufaehs".to_string());
    assert_eq!(encrypt::gen_pass(&base_method), "dmFyaWFibGVwYXNz".to_string());
    assert_eq!(encrypt::gen_pass(&enig_method), "HTKWGIYLBKQA");
    }

}
