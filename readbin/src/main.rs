use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match fs::read(&args[1]) {
            Ok(data) => {
                headers::elf64::Elf64::from_bytes(&data);
            }
            Err(err) => println!("Error reading binary: {}", err),
        },
        _ => {
            println!("usage: {:?} <binary file>", args[0]);
        }
    }
}
