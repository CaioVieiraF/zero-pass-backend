use super::*;

#[test]
fn methods_test() {
    let uw: String = String::from("uniquepass");
    let vw = String::from("variablepass");

    assert_eq!(
        encrypt::Vigenere.encrypt(uw.clone(), vw.clone()),
        Ok("pnzyufaehs".to_string())
    );
    assert_eq!(
        encrypt::Base64.encrypt(uw.clone(), vw.clone()),
        Ok("dW5pcXVlcGFzcw==".to_string())
    );
    assert_eq!(encrypt::Xor.encrypt(uw, vw), Ok("dpyuheds".to_string()));
}

#[test]
fn gen_for_methods_test() {
    let builder = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass");
    //let xor_method = Methods::Xor(data.clone());

    assert_eq!(
        builder
            .clone()
            .method(Base64)
            .unwrap()
            .method(Vigenere)
            .unwrap()
            .build(),
        "yw5gkxwwgvfrur=="
    );
    assert_eq!(
        builder
            .clone()
            .method(Vigenere)
            .unwrap()
            .method(Base64)
            .unwrap()
            .build(),
        "cG56eXVmYWVocw=="
    );
}

#[test]
fn gen_test() {
    let builder = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass");

    assert_eq!(
        builder.clone().method(Vigenere).unwrap().build(),
        "pnzyufaehs"
    );
    assert_eq!(
        builder.clone().method(Base64).unwrap().build(),
        "dW5pcXVlcGFzcw=="
    );
    assert_eq!(builder.clone().method(Xor).unwrap().build(), "dpyuheds");
}

#[test]
fn gen_repeat_pass() {
    let builder = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass");

    assert_eq!(
        builder.clone().repeat(2).method(Vigenere).unwrap().build(),
        "knqgugliws"
    );
    assert_eq!(
        builder.clone().repeat(2).method(Base64).unwrap().build(),
        "ZFc1cGNYVmxjR0Z6Y3c9PQ=="
    );
    assert_eq!(
        builder.clone().repeat(2).method(Xor).unwrap().build(),
        "srljhiw"
    );
}

#[test]
fn get_methods() {
    let methods = Methods::get_methods();
    println!("{:?}", methods);

    assert_eq!(methods[0], "Vigenere");

    let method = Methods::get_method(methods[1]);
    let method = method.unwrap();

    let base = method.encrypt("uniquepass".to_string(), "variablepass".to_string());
    assert_eq!(base, Ok("dW5pcXVlcGFzcw==".to_string()))
}

#[test]
fn gen_from_get() {
    let method = Methods::get_method("Vigenere");
    let method = method.unwrap();

    let pass = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass")
        .method_ptr(method).unwrap()
        .build();

    assert_eq!(pass, "pnzyufaehs");
}
