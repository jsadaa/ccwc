#[derive(Debug)]
pub enum Opt {
    Bytes,
    Lines,
    Words,
    Chars,
    All
}

pub fn get_opt(s: &str) -> Result<Opt, String> {
    match s {
        "-c" => Ok(Opt::Bytes),
        "-l" => Ok(Opt::Lines),
        "-w" => Ok(Opt::Words),
        "-m" => Ok(Opt::Chars),
        "-clw" => Ok(Opt::All),
        _ => Err(format!("illegal option -- {}", s.replace('-', "")))
    }
}