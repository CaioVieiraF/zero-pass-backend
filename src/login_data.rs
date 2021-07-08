use crate::Methods;
use crate::encrypt;
use crate::LoginData;

pub fn new (method: Methods) -> LoginData {
    LoginData {
        cpw: encrypt::gen_pass(&method),
        symetric_method: method,
    }
}

#[test]
fn login_test(){

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

    assert_eq!(new(vige_method.clone()), LoginData {
        symetric_method: vige_method.clone(),
        cpw: "pnzyufaehs".to_string()
    });
    assert_eq!(new(base_method.clone()), LoginData {
        symetric_method: base_method.clone(),
        cpw: "dmFyaWFibGVwYXNz".to_string()
    });
    assert_eq!(new(enig_method.clone()), LoginData {
        symetric_method: enig_method.clone(),
        cpw: format!("HTKWGIYLBKQA")
    });
}

