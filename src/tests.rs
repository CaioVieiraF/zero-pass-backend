use super::*;

#[test]
fn methods_test() {
    let uw: String = String::from("uniquepass");
    let vw: String = String::from("variablepass");

    assert_eq!(
        encrypt::Vigenere.encrypt(&uw, &vw),
        Ok("pnzyufaehs".to_string())
    );
    assert_eq!(
        encrypt::Base64.encrypt(&uw, &vw),
        Ok("dW5pcXVlcGFzcw==".to_string())
    );
    assert_eq!(encrypt::Xor.encrypt(&uw, &vw), Ok("dpyuheds".to_string()));
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
            .method(Base64).unwrap()
            .method(Vigenere).unwrap()
            .build(),
        "yw5gkxwwgvfrur=="
    );
    assert_eq!(
        builder
            .clone()
            .method(Vigenere).unwrap()
            .method(Base64).unwrap()
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
    assert_eq!(
        builder.clone().method(Xor).unwrap().build(),
        "dpyuheds"
    );

}

#[test]
fn gen_repeat_pass() {
    let builder = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass");

    assert_eq!(
        builder.clone()
            .repeat(2)
            .method(Vigenere).unwrap()
            .build(),
        "knqgugliws"
    );
    assert_eq!(
        builder.clone()
            .repeat(2)    
            .method(Base64).unwrap()
            .build(),
        "ZG1GeWFXRmliR1Z3WVhOeg=="
    );
    assert_eq!(
        builder.clone()
            .repeat(2)
            .method(Xor).unwrap()
            .build(),
        "srljhiw"
    );

}
