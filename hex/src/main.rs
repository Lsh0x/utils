use clap::{App, Arg};

use std::i64;

fn hex_as_decimal(str: &str) -> Result<i64, std::num::ParseIntError> {
    return i64::from_str_radix(str, 16);
}

fn format_str(str: &str) -> &str {
    if str.starts_with("0x") {
        return &str[2..];
    }
    if str.ends_with("x0") {
        return &str[0..str.len() - 2];
    }
    return str;
}

fn hex_to_decimal(str: &str) {
    match hex_as_decimal(format_str(str)) {
        Ok(hex) => println!("{:?}", hex),
        Err(err) => println!("Err {:?}", err),
    }
}

fn hex_to_be(str: &str) {
    match hex_as_decimal(format_str(str)) {
        Ok(hex) => println!("{:#x}", hex.to_be()),
        Err(err) => println!("Err {:?}", err),
    }
}

fn hex_to_le(str: &str) {
    match hex_as_decimal(format_str(&str.chars().rev().collect::<String>()[..])) {
        Ok(hex) => println!("{:#x}", hex),
        Err(err) => println!("Err {:?}", err),
    }
}

fn hex_to_be_payload(str: &str) {
    match hex_as_decimal(format_str(&str.chars().rev().collect::<String>()[..])) {
        Ok(hex) => {
            let payload: String = hex
                .to_be_bytes()
                .chunks(1)
                .map(|item| format!("\\x{:02x}", item[0]))
                .collect();
            println!("{:?}", payload);
        }
        Err(err) => println!("Err {:?}", err),
    }
}

fn main() {
    let matches = App::new("hex")
        .version("1.0")
        .author("LSH. <github@lsh.tech>")
        .about("Does awesome things with hexadecimal")
        .subcommand(
            App::new("to")
                .about("convert hex to ...")
                .version("0.0.1")
                .author("LSH. <github@lsh.tech>")
                .subcommand(
                    App::new("be").about("convert hex to big endian").arg(
                        Arg::new("hex")
                            .about("hexadecimal to transform into big endian")
                            .required(true),
                    ),
                )
                .subcommand(
                    App::new("le").about("convert hex to big endian").arg(
                        Arg::new("hex")
                            .about("hexadecimal to transform into little endian")
                            .required(true),
                    ),
                )
                .subcommand(
                    App::new("decimal").about("convert hex to decimal").arg(
                        Arg::new("hex")
                            .about("hexadecimal to transform into decimal")
                            .required(true),
                    ),
                )
                .subcommand(
                    App::new("be_payload")
                        .about("convert hex to an big endian payload")
                        .arg(
                            Arg::new("hex")
                                .about("hexadecimal to transform into decimal")
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            App::new("substraction")
                .about("substract to hex")
                .version("0.0.1")
                .author("LSH. <github@lsh.tech>")
                .arg(
                    Arg::new("hex1")
                        .about("hexadecimal to substact to")
                        .required(true),
                )
                .arg(
                    Arg::new("hex2")
                        .about("hexadecimal to substract")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("to") {
        if let Some(matches) = matches.subcommand_matches("decimal") {
            return hex_to_decimal(matches.value_of("hex").unwrap());
        }
        if let Some(matches) = matches.subcommand_matches("be") {
            return hex_to_be(matches.value_of("hex").unwrap());
        }
        if let Some(matches) = matches.subcommand_matches("le") {
            return hex_to_le(matches.value_of("hex").unwrap());
        }
        if let Some(matches) = matches.subcommand_matches("be_payload") {
            return hex_to_be_payload(matches.value_of("hex").unwrap());
        }
        if let Some(matches) = matches.subcommand_matches("substraction") {
            return hex_to_decimal(matches.value_of("hex").unwrap());
        }
    }
}
