use crate::{CipherResult, CipherError};

pub fn xor(uw: &str, vw: &str) -> CipherResult {
    //let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut binary_vw_word = "".to_string();
    let mut binary_uw_word = "".to_string();
    let new_pass: String;

    for i in &mut (*vw).bytes() {
        binary_vw_word += &format!("0{:b}", i);
    }
    for i in &mut (*uw).bytes() {
        binary_uw_word += &format!("0{:b}", i);
    }

    println!("{:?}", binary_uw_word);
    println!("{:?}", binary_vw_word);

    let mut x = 0;
    let mut binary_pass = String::new();
    for i in 0..uw.len() {
        if x == 8 {
            binary_pass += " ";
            x = 0
        }
        let uw_val = &(binary_uw_word.as_bytes()[{
            if i > binary_uw_word.len() - 1 {
                i - (binary_uw_word.len() * (i / binary_uw_word.len() as usize))
            } else {
                i
            }
        }] as char)
            .to_string()
            .parse::<u8>()
            .unwrap();
        let vw_val = &(binary_vw_word.as_bytes()[i] as char)
            .to_string()
            .parse()
            .unwrap();
        binary_pass += &format!("0{:b}", uw_val ^ vw_val);
        x += 1;
    }

    //let binary_vec: Vec<&str> = binary_pass.split(" ").collect();
    new_pass = std::str::from_utf8(binary_pass.as_bytes())
        .unwrap()
        .to_string();
    //for i in binary_vec{
    //let number = usize::from_str_radix(i, 2).unwrap();
    //new_pass += &(alphabet.as_bytes()[number] as char).to_string();
    //}

    println!("{:?}", binary_pass);

    Ok(new_pass)
}
