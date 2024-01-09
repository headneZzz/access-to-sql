use std::{io, process};

pub fn exit_gracefully_with_error_code() -> ! {
    println!("Для продолжения нажмите Enter...");
    match io::stdin().read_line(&mut "".to_string()) {
        Ok(_c) => process::exit(1),
        Err(_e) => process::exit(1)
    }
}
