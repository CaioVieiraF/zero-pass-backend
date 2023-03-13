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
    let vige_method = Vigenere;
    let base_method = Base64;
    //let xor_method = Methods::Xor(data.clone());

    assert_eq!(
        builder
            .clone()
            .method(&base_method)
            .method(&vige_method)
            .build(),
        Ok("yw5gkxwwgvfrur==".to_string())
    );
    assert_eq!(
        builder
            .clone()
            .method(&vige_method)
            .method(&base_method)
            .build(),
        Ok("cG56eXVmYWVocw==".to_string())
    );
}

#[test]
fn gen_test() {
    let builder = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass");
    let vige_method = Vigenere;
    let base_method = Base64;
    let xor_method = Xor;

    assert_eq!(
        builder.clone().method(&vige_method).build(),
        Ok("pnzyufaehs".to_string())
    );
    assert_eq!(
        builder.clone().method(&base_method).build(),
        Ok("dW5pcXVlcGFzcw==".to_string())
    );
    assert_eq!(
        builder.clone().method(&xor_method).build(),
        Ok("dpyuheds".to_string())
    );

    let repeat_builder = builder.repeat(2);

    assert_eq!(
        repeat_builder.clone().method(&vige_method).build(),
        Ok("knqgugliws".to_string())
    );
    assert_eq!(
        repeat_builder.clone().method(&base_method).build(),
        Ok("ZG1GeWFXRmliR1Z3WVhOeg==".to_string())
    );
    assert_eq!(
        repeat_builder.clone().method(&xor_method).build(),
        Ok("srljhiw".to_string())
    );
}
