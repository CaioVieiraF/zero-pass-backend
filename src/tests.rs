use super::*;

#[test]
fn scramble_words() {
    let context_word = "test";
    let service_word = "test";
    let scrambled_word = encrypt::Xor.encrypt(context_word.as_bytes(), service_word.as_bytes());
    let expected_result: Box<[u8]> = Box::new([0; 4]);

    assert!(scrambled_word.is_ok());
    assert_eq!(scrambled_word.unwrap(), expected_result);

    let scrambled_word =
        encrypt::Vigenere.encrypt(context_word.as_bytes(), service_word.as_bytes());
    let expected_result = "mikm";
    assert!(scrambled_word.is_ok());
    assert_eq!(scrambled_word.unwrap().as_ref(), expected_result.as_bytes());
}

#[test]
fn gen_repeat_pass() {
    let builder = encrypt::PasswordBuilder::new()
        .context("uniquepass")
        .service("variablepass");
    let mut method = hash_from_string("sha3_256").unwrap();

    let vigenere_test = builder
        .clone()
        .encrypt(Vigenere)
        .unwrap()
        .repeat(3)
        .hash(&mut *method)
        .encode();

    assert_eq!(
        vigenere_test,
        "QrK/Ls2tCbawNVOliUe29fy7qo6FF+8Lv4dK6wahUwo="
    );

    // with `repeat(2)`: "d9w8s+bI1rHrOBAeOO2XV+GfNf1lDoC962KuhdLsjYM="
}

#[test]
fn gen_pass() {
    let builder = encrypt::PasswordBuilder::new()
        .context("uniquepass")
        .service("variablepass");
    let mut method = hash_from_string("sha3_256").unwrap();

    let vigenere_test = builder
        .clone()
        .encrypt(Vigenere)
        .unwrap()
        .hash(&mut *method)
        .encode();

    assert_eq!(
        vigenere_test,
        "sAs7g41UDozsClhX+1kj7p+9H+gfbnoXikrivBqDDgQ="
    );
}

#[test]
fn get_method() {
    let method_sha3_256 = hash_from_string("sha3_256");
    let method_that_does_not_exist = hash_from_string("something");

    assert!(method_sha3_256.is_ok());
    assert!(method_that_does_not_exist.is_err());
}
