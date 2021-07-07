use crate::Methods;
use crate::encrypt;
use crate::LoginData;

pub fn new (method: Methods, unique_word: String, variable_word: String) -> LoginData {
    LoginData {
        cpw: encrypt::gen_pass(&method, &unique_word, &variable_word),
        symetric_method: method,
        suw: unique_word,
        svw: variable_word,
    }
}

#[test]
fn login_test(){

    let method_enum = vec![
        Methods::Vigenere,
        Methods::B64,
        Methods::Xor,
        Methods::Enigma,
    ];
    let uw: String = String::from("uniquepass");
    let vw: String = String::from("variablepass");

    assert_eq!(new(method_enum[0].clone(), "longer".to_string(), "short".to_string()), LoginData {
        symetric_method: method_enum[0].clone(),
        suw: "longer".to_string(),
        svw: "short".to_string(),
        cpw: "dvbxxj".to_string()
    });
    assert_eq!(new(method_enum[0].clone(), uw.clone(), vw.clone()), LoginData {
        symetric_method: method_enum[0].clone(),
        suw: uw.clone(),
        svw: vw.clone(),
        cpw: "pnzyufaehs".to_string()
    });
    assert_eq!(new(method_enum[1].clone(), vw.clone(), "".to_string()), LoginData {
        symetric_method: method_enum[1].clone(),
        suw: vw.clone(),
        svw: "".to_string(),
        cpw: "dmFyaWFibGVwYXNz".to_string()
    });
    assert_eq!(new(method_enum[2].clone(), uw.clone(), vw.clone()), LoginData {
        symetric_method: method_enum[2].clone(),
        suw: uw.clone(),
        svw: vw.clone(),
        cpw: format!("{}, {}", &uw, &vw)
    });
    assert_eq!(new(method_enum[3].clone(), uw.clone(), vw.clone()), LoginData {
        symetric_method: method_enum[3].clone(),
        suw: uw.clone(),
        svw: vw.clone(),
        cpw: format!("{}, {}", &uw, &vw)
    });
}

