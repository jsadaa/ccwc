use std::io;
use std::io::{BufRead, Read};

pub fn count_bytes(source: &mut dyn Read) -> usize {
    source.bytes().filter(|x| x.is_ok()).count()
}

pub fn count_lines(source: &mut dyn Read) -> usize {
    let reader = io::BufReader::new(source);
    reader.lines().count() - 1
}

pub fn count_words(source: &mut dyn Read) -> usize {
    let mut total_words: usize = 0;

    for line in io::BufReader::new(source).lines() {
        total_words += line.unwrap_or(String::from(""))
            .split_whitespace()
            .collect::<Vec<&str>>()
            .len();
    }

    total_words
}

pub fn count_chars(source: &mut dyn Read) -> usize {
    let mut contents = String::new();
    source.read_to_string(&mut contents).expect("Failed to read from file");
    contents.chars().count()
}