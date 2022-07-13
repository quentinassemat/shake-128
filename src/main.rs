mod helpers;

use crate::helpers::*;
use hex::encode;
use std::env;
use std::io;
use std::io::prelude::*;

// mettre fonction qui check la validité des arguments en entrée.
fn main() {
    // input handling
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage : ./shake128 <output lenght> <input (nom du fichier)>");
        return;
    }
    match args[1].parse::<u64>() {
        Ok(output_len) => {
            let mut input: Vec<u8> = Vec::new();
            io::stdin()
                .read_to_end(&mut input)
                .expect("Can't read stdin");

            let output = shake128(&mut input, output_len);
            println!("{}", encode(output));
        }
        Err(_e) => eprintln!("Erreur input"),
    }
}
