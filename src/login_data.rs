use crate::encrypt;
use crate::LoginData;
use crate::Methods;

pub fn new(method: Methods) -> LoginData {
    LoginData {
        cpw: encrypt::gen_pass(&method, None),
        symetric_method: method,
    }
}

#[test]
fn login_test() {
    let data = encrypt::MethodArgs {
        word: "uniquepass",
        password: "variablepass",
    };

    let vige_method = Methods::Vigenere(data.clone());
    let base_method = Methods::B64(data.clone());
    let xor_method = Methods::Xor(data.clone());

    assert_eq!(
        new(vige_method.clone()),
        LoginData {
            symetric_method: vige_method.clone(),
            cpw: Ok("pnzyufaehs".to_string())
        }
    );
    assert_eq!(
        new(base_method.clone()),
        LoginData {
            symetric_method: base_method.clone(),
            cpw: Ok("dmFyaWFibGVwYXNz".to_string())
        }
    );
    assert_eq!(
        new(xor_method.clone()),
        LoginData {
            symetric_method: xor_method.clone(),
            cpw: Ok("dpyuheds".to_string())
        }
    );
}
