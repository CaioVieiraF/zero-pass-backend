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
    
    let data = encrypt::MethodArgs { word: "uniquepass", password: "variablepass" };

    let vige_method = Methods::Vigenere(data.clone());
    let base_method = Methods::B64(data.clone());
    let _xor_method = Methods::Xor(data.clone());

    assert_eq!(new(vige_method.clone()), LoginData {
        symetric_method: vige_method.clone(),
        cpw: "pnzyufaehs".to_string()
    });
    assert_eq!(new(base_method.clone()), LoginData {
        symetric_method: base_method.clone(),
        cpw: "dmFyaWFibGVwYXNz".to_string()
    });
}

