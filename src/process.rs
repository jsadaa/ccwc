use std::path::Path;
use crate::options::Opt;
use crate::parser;

pub fn execute(opt: Opt, path: &Path) -> Result<String, String> {
    match opt {
        Opt::Bytes => {
            match parser::count_bytes(path) {
                Ok(res) => {
                    Ok(res.to_string())
                },
                Err(msg) => {
                    Err(msg)
                }
            }
        },
        Opt::Lines => {
            match parser::count_lines(path) {
                Ok(res) => {
                    Ok(res.to_string())
                },
                Err(msg) => {
                    Err(msg)
                }
            }
        },
        Opt::Words => {
            match parser::count_words(path) {
                Ok(res) => {
                    Ok(res.to_string())
                },
                Err(msg) => {
                    Err(msg)
                }
            }
        },
        Opt::Characters => {
            match parser::count_chars(path) {
                Ok(res) => {
                    Ok(res.to_string())
                },
                Err(msg) => {
                    Err(msg)
                }
            }
        },
        Opt::All => {
            let mut results: Vec<Result<String, String>> = vec![execute(Opt::Lines, path)];
            results.push(execute(Opt::Words, path));
            results.push(execute(Opt::Bytes, path));

            let mut err: Option<Result<String, String>> = None;
            let mut format: String = String::from("");

            for result in results {
                match result {
                    Ok(res) => format = format!("{}{:>8}", format, res),
                    Err(msg) => err = Some(Err(msg))
                }
            }

            if let Some(res) = err{
                Err(res.unwrap())
            } else {
                Ok(format)
            }
        },
    }
}