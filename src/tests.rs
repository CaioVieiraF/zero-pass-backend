use super::*;

#[test]
fn methods_test() {
    let uw: String = String::from("uniquepass");
    let vw: String = String::from("variablepass");

    assert_eq!(encrypt::vigenere(&uw, &vw), Ok("pnzyufaehs".to_string()));
    assert_eq!(encrypt::b64(&vw), Ok("dmFyaWFibGVwYXNz".to_string()));
    assert_eq!(encrypt::xor(&uw, &vw), Ok("dpyuheds".to_string()));
}

#[test]
fn gen_for_methods_test() {
    let data = encrypt::MethodArgs {
        word: "uniquepass",
        password: "variablepass",
    };
    let vige_method = Methods::Vigenere(data.clone());
    let base_method = Methods::B64(data.clone());
    //let xor_method = Methods::Xor(data.clone());

    let mut methods_vec: Vec<&Methods> = Vec::new();
    let mut methods_vec2: Vec<&Methods> = Vec::new();
    methods_vec.push(&base_method);
    methods_vec.push(&vige_method);

    methods_vec2.push(&vige_method);
    methods_vec2.push(&base_method);

    let test = encrypt::gen_pass_for_methods(methods_vec, vec![None, None]);
    let test2 = encrypt::gen_pass_for_methods(methods_vec2, vec![None, None]);
    assert_eq!(test, Ok("ymwgaxqmqgnotxeh".to_string()));
    assert_eq!(test2, Ok("cG56eXVmYWVocw==".to_string()));
}

#[test]
fn gen_test() {
    let data = encrypt::MethodArgs {
        word: "uniquepass",
        password: "variablepass",
    };
    let vige_method = Methods::Vigenere(data.clone());
    let base_method = Methods::B64(data.clone());
    let xor_method = Methods::Xor(data.clone());

    assert_eq!(
        encrypt::gen_pass(&vige_method, None),
        Ok("pnzyufaehs".to_string())
    );
    assert_eq!(
        encrypt::gen_pass(&base_method, None),
        Ok("dmFyaWFibGVwYXNz".to_string())
    );
    assert_eq!(
        encrypt::gen_pass(&xor_method, None),
        Ok("dpyuheds".to_string())
    );

    assert_eq!(
        encrypt::gen_pass(&vige_method, Some(2)),
        Ok("knqgugliws".to_string())
    );
    assert_eq!(
        encrypt::gen_pass(&base_method, Some(2)),
        Ok("ZG1GeWFXRmliR1Z3WVhOeg==".to_string())
    );
    assert_eq!(
        encrypt::gen_pass(&xor_method, Some(2)),
        Ok("srljhiw".to_string())
    );
}

#[test]
fn hashmap_test<'a>() {
    let methods: HashMap<String, fn(MethodArgs<'a>) -> Methods<'a>> = crate::get_methods();

    for (key, _) in methods.iter() {
        let get_func = methods.get(key).unwrap();
        let _method: Methods = get_func(MethodArgs {
            word: "a",
            password: "b",
        });
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
        login_data::new(vige_method.clone()),
        login_data::LoginData {
            symetric_method: vige_method.clone(),
            cpw: Ok("pnzyufaehs".to_string())
        }
    );
    assert_eq!(
        login_data::new(base_method.clone()),
        login_data::LoginData {
            symetric_method: base_method.clone(),
            cpw: Ok("dmFyaWFibGVwYXNz".to_string())
        }
    );
    assert_eq!(
        login_data::new(xor_method.clone()),
        login_data::LoginData {
            symetric_method: xor_method.clone(),
            cpw: Ok("dpyuheds".to_string())
        }
    );
}
