mod options;
mod parser;
mod process;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_1: &str;
    let arg_2: &str;

    match args.len() {
        2 => {
            arg_1 = "-clw";
            arg_2 = args.get(1).unwrap();
        },
        3 => {
            arg_1 = args.get(1).unwrap();
            arg_2 = args.get(2).unwrap();
        }
        _ => {
            println!("usage: {} [-clwm] [file ...]", args[0]);
            return;
        }
    }

    let file = Path::new(arg_2);

    let opt = match options::get_opt(arg_1) {
        Ok(res) => {
            res
        },
        Err(msg) => {
            println!("{}: {}", args[0], msg);
            println!("usage: {} [-clwm] [file ...]", args[0]);
            return;
        }
    };

    if !file.exists() {
        println!("{}: {}: open: No such file or directory", args[0], arg_2);
        return;
    }

    let res: Result<String, String> = process::execute(opt, file);

    match res {
        Ok(s) => {
            println!("{:>8} {}", s, arg_2)
        },
        Err(msg) => {
            println!("{} {}", args[0], msg)
        }
    }
}
