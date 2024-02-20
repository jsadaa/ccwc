use std::io::{Cursor, Read};
use crate::options::Opt;
use crate::parser;

fn create_buffer(source: &mut dyn Read) -> Cursor<Vec<u8>> {
    let mut buffer = Vec::new();
    source.read_to_end(&mut buffer).unwrap();
    Cursor::new(buffer)
}

pub fn execute(opt: Opt, source: &mut dyn Read) -> String {
    match opt {
        Opt::Bytes => {
            parser::count_bytes(source).to_string()
        },
        Opt::Lines => {
            parser::count_lines(source).to_string()
        },
        Opt::Words => {
            parser::count_words(source).to_string()
        },
        Opt::Chars => {
            parser::count_chars(source).to_string()
        }
        Opt::All => {
            let buffer = create_buffer(source);
            let results = [
                execute(Opt::Lines, &mut buffer.clone()),
                execute(Opt::Words, &mut buffer.clone()),
                execute(Opt::Bytes, &mut buffer.clone()),
            ];

            results.iter().fold(String::new(), |acc, x| format!("{}{:>8}", acc, x))
        }
    }
}