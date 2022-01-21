use std::{
    io::Read,
    fs::{
        File,
        self
    }
};
use crate::save::Save;

mod save;

#[test]
fn test() {
    let path = std::env::var("SAV").unwrap();
    let mut sav = File::open(&path).unwrap();
    let mut sav_string = String::new();
    sav.read_to_string(&mut sav_string).unwrap();
    let decrypted_sav: String = decrypt(&sav_string);
    fs::write(format!("{}.decrypt", &path), &decrypted_sav).unwrap();
    let save: Save = match toml::from_str(&decrypted_sav) {
        Ok(save) => dbg!(save),
        Err(e) => panic!("{}", e)
    };
}

const ENCRYPTION_KEY: &[u8] = b"QWERTY";

fn decrypt(input: &str) -> String {
    let mut output = String::new();
    let mut buf = Vec::new();
    let mut is_header = false;
    for c in input.chars() {
        match c {
            '['|']'|'='|'"'|'\n'|'\r' => {
                if !buf.is_empty() {
                    for (i, c) in buf.drain(..).enumerate() {
                        let offset = ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()];
                        let decrypt_c = char::from_u32(c as u32 - offset as u32).unwrap();
                        if is_header && decrypt_c.is_digit(10) {
                            output.push('.');
                            is_header = false;
                        }
                        else if decrypt_c == '"' {
                            output.push('\\');
                        }
                        output.push(char::from_u32(c as u32 - offset as u32).unwrap());
                    }
                    is_header = false;
                }
                output.push(c);
                if c == '[' { is_header = true; }
            },
            '\0' => {},
            c => buf.push(c),
        }
    }
    output
}