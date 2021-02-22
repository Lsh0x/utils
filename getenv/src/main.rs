use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match env::var(&args[1]) {
            Ok(env) => println!("addr: {:p}", &env),
            Err(_) => println!("env variable not found"),
        },
        _ => println!("usage: {:?} <env variable>", args[0]),
    }
}
