mod options;
mod parser;
mod process;

use std::{env, io};
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use crate::options::Opt;

fn main() {
    let mut args = env::args().skip(1);

    let arg_1 = match args.next() {
        Some(arg) => arg,
        None => {
            println!("Usage: {} [-clwm] [file ...]", env::args().next().unwrap());
            return;
        }
    };

    let (opt, filename) = match options::get_opt(&arg_1) {
        Ok(res) => (res, args.next()),
        Err(_) => (Opt::All, Some(arg_1)),
    };

    match filename {
        Some(filename) => {
            let path = Path::new(&filename);
            if !path.exists() {
                println!("{}: {}: open: No such file or directory", env::args().next().unwrap(), filename);
                return;
            }

            if let Ok(mut file) = File::open(path) {
                let res: String = process::execute(opt, &mut file);
                println!("{} {}", res, filename);
            } else {
                println!("{}: {}: open: Permission denied", env::args().next().unwrap(), filename);
            }
        }
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
            let mut fake_file = Cursor::new(buffer);

            let res: String = process::execute(opt, &mut fake_file);

            println!("{}", res);
        }
    };
}
