use crate::encrypt;
use crate::CipherResult;
use crate::Methods;

#[derive(Debug, Clone, PartialEq)]
pub struct LoginData<'a> {
    pub symetric_method: Methods<'a>,
    pub cpw: CipherResult,
}

pub fn new(method: Methods) -> LoginData {
    LoginData {
        cpw: encrypt::gen_pass(&method, None),
        symetric_method: method,
    }
}
