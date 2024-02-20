use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, Read};
use std::path::Path;

pub fn count_bytes(path: &Path) -> Result<usize, String> {
    let file= File::open(path);
    let mut total_bytes: usize = 0;

    if let Ok(file) = file {
        for byte in file.bytes() {
            if byte.is_ok() {
                total_bytes += 1;
            }
        }
        Ok(total_bytes)
    } else {
        Err(format!("{:?}: open: No such file or directory", path.to_str().unwrap_or_default()))
    }
}

pub fn count_lines(path: &Path) -> Result<usize, String> {
    let file = File::open(path);

    if let Ok(file) = file {
        let reader = io::BufReader::new(file);
        Ok(reader.lines().count() - 1)
    } else {
        Err(format!("{:?}: open: No such file or directory", path.to_str().unwrap_or_default()))
    }
}

pub fn count_words(path: &Path) -> Result<usize, String> {
    let file = File::open(path);
    let mut total_words: usize = 0;

    if let Ok(file) = file {
        for line in io::BufReader::new(file).lines() {
            total_words += line.unwrap_or(String::from(""))
                .split_whitespace()
                .collect::<Vec<&str>>()
                .len();
        }

        Ok(total_words)
    } else {
        Err(format!("{:?}: open: No such file or directory", path.to_str().unwrap_or_default()))
    }
}

pub fn count_chars(path: &Path) -> Result<usize, String> {
    match fs::read_to_string(path) {
        Ok(file_contents) => Ok(file_contents.chars().count()),
        Err(_) => Err(format!("{:?}: open: No such file or directory", path.to_str().unwrap_or_default()))
    }
}