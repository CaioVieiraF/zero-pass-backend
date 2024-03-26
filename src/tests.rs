use super::*;
use std::sync::Arc;
use tokio;

#[tokio::test]
async fn methods_test() {
    let uw: Arc<str> = Arc::from("uniquepass");
    let vw: Arc<str> = Arc::from("variablepass");

    assert_eq!(
        encrypt::Vigenere.encrypt(uw.clone(), vw.clone()).await,
        Ok("pnzyufaehs".to_string())
    );
    assert_eq!(
        encrypt::Base64.encrypt(uw.clone(), vw.clone()).await,
        Ok("dW5pcXVlcGFzcw==".to_string())
    );
    assert_eq!(encrypt::Xor.encrypt(uw, vw).await, Ok("dpyuheds".to_string()));
}

#[tokio::test]
async fn gen_for_methods_test() {
    let builder = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass");
    //let xor_method = Methods::Xor(data.clone());

    assert_eq!(
        builder
            .clone()
            .method(Base64)
            .await
            .unwrap()
            .method(Vigenere)
            .await
            .unwrap()
            .build(),
        "yw5gkxwwgvfrur=="
    );
    assert_eq!(
        builder
            .clone()
            .method(Vigenere)
            .await
            .unwrap()
            .method(Base64)
            .await
            .unwrap()
            .build(),
        "cG56eXVmYWVocw=="
    );
}

#[tokio::test]
async fn gen_test() {
    let builder = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass");

    assert_eq!(
        builder.clone().method(Vigenere).await.unwrap().build(),
        "pnzyufaehs"
    );
    assert_eq!(
        builder.clone().method(Base64).await.unwrap().build(),
        "dW5pcXVlcGFzcw=="
    );
    assert_eq!(builder.clone().method(Xor).await.unwrap().build(), "dpyuheds");
}

#[tokio::test]
async fn gen_repeat_pass() {
    let builder = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass");

    assert_eq!(
        builder.clone().repeat(2).method(Vigenere).await.unwrap().build(),
        "knqgugliws"
    );
    assert_eq!(
        builder.clone().repeat(2).method(Base64).await.unwrap().build(),
        "ZFc1cGNYVmxjR0Z6Y3c9PQ=="
    );
    assert_eq!(
        builder.clone().repeat(2).method(Xor).await.unwrap().build(),
        "srljhiw"
    );
}

#[tokio::test]
async fn get_methods() {
    let methods = Methods::get_methods();
    println!("{:?}", methods);

    assert_eq!(methods[0], "Vigenere");

    let method = Methods::try_from(methods[1]);
    let method = method.unwrap().to_method();

    let base = method.encrypt(Arc::from("uniquepass"), Arc::from("variablepass")).await;
    assert_eq!(base, Ok("dW5pcXVlcGFzcw==".to_string()))
}

#[tokio::test]
async fn gen_from_try_from() {
    let method = Methods::try_from("Vigenere");
    let method = method.unwrap().to_method();

    let pass = encrypt::PasswordBuilder::new()
        .unique("uniquepass")
        .variable("variablepass")
        .method_ptr(method)
        .await
        .unwrap()
        .build();

    assert_eq!(pass, "pnzyufaehs");
}

#[tokio::test]
async fn async_heavy_task() {
    let pass = encrypt::PasswordBuilder::new()
        .unique("test")
        .variable("test")
        .repeat(64)
        .method(Base64)
        .await
        .unwrap()
        .build();
}
