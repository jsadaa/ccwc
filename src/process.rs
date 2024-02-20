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
            let buffer = create_buffer(source);
            let result = parser::count_bytes(&mut buffer.clone());
            format!("{:>8}", result)
        },
        Opt::Lines => {
            let buffer = create_buffer(source);
            let result = parser::count_lines(&mut buffer.clone());
            format!("{:>8}", result)
        },
        Opt::Words => {
            let buffer = create_buffer(source);
            let result = parser::count_words(&mut buffer.clone());
            format!("{:>8}", result)
        },
        Opt::Chars => {
            let buffer = create_buffer(source);
            let result = parser::count_chars(&mut buffer.clone());
            format!("{:>8}", result)
        }
        Opt::All => {
            let buffer = create_buffer(source);
            let results = [
                execute(Opt::Lines, &mut buffer.clone()),
                execute(Opt::Words, &mut buffer.clone()),
                execute(Opt::Bytes, &mut buffer.clone()),
            ];

            format!("{:>8}{:>8}{:>8}", results[0], results[1], results[2])
        }
    }
}