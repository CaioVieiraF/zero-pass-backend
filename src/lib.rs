use std::collections::HashMap;
use crate::encrypt::{ Methods, MethodArgs };

pub mod encrypt;
pub mod login_data;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct LoginData<'a>{
    symetric_method: Methods<'a>,
    pub cpw: String
}

pub fn get_methods<'a>() -> HashMap<String, fn(MethodArgs<'a>) -> Methods<'a>> {
    let mut methods: HashMap<String, fn(MethodArgs<'a>) -> Methods<'a>> = HashMap::new();

    methods.insert(String::from("Vigenere"), Methods::Vigenere);
    methods.insert(String::from("Base64"), Methods::B64);
    methods.insert(String::from("Xor"), Methods::Xor);

    methods
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn methods_test() {
        let uw: String = String::from("uniquepass");
        let vw: String = String::from("variablepass");

        assert_eq!(encrypt::vigenere(&uw, &vw), "pnzyufaehs");
        assert_eq!(encrypt::b64(&vw), "dmFyaWFibGVwYXNz");
        assert_eq!(encrypt::xor(&uw, &vw), format!("{}, {}", &uw, &vw));
    }

    #[test]
    fn gen_test(){
        let data = encrypt::MethodArgs{ word: "uniquepass", password: "variablepass" };
        let vige_method = Methods::Vigenere(data.clone());
        let base_method = Methods::B64(data.clone());
        let _xor_method = Methods::Xor(data.clone());

        assert_eq!(encrypt::gen_pass(&vige_method), "pnzyufaehs".to_string());
        assert_eq!(encrypt::gen_pass(&base_method), "dmFyaWFibGVwYXNz".to_string());
    }

    #[test]
    fn hashmap_test<'a>() {
        let methods: HashMap<String, fn(MethodArgs<'a>) -> Methods<'a>> = crate::get_methods();
        
        for (key, _) in methods.iter() {
            let get_func = methods.get(key).unwrap();
            let _method: Methods = get_func(
                    MethodArgs { word: "a", password: "b" }
                );
        }
    }

}
