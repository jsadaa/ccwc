use std::io::Read;

pub fn count_bytes(source: &mut dyn Read) -> usize {
    source.bytes().filter(|x| x.is_ok()).count()
}

pub fn count_lines(source: &mut dyn Read) -> usize {
    let mut buffer = [0; 4096];
    let mut count = 0;

    while let Ok(n) = source.read(&mut buffer) {
        if n == 0 { break; }
        count += buffer[..n].iter().filter(|&&c| c == b'\n').count();
    }

    count
}

pub fn count_words(source: &mut dyn Read) -> usize {
    let mut buffer = [0; 4096]; // Augmenter la taille du buffer pour réduire les lectures.
    let mut wc = 0;
    let mut is_prev_space = true;

    while let Ok(bytes_read) = source.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        for &b in &buffer[0..bytes_read] {
            let is_space = is_space(b);
            if is_space && !is_prev_space {
                wc += 1;
            }
            is_prev_space = is_space;
        }
    }

    if !is_prev_space {
        wc += 1;
    }

    wc
}

fn is_space(b: u8) -> bool {
    b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' || b == 12 || b == 11
}

pub fn count_chars(source: &mut dyn Read) -> usize {
    let mut buffer = Vec::new();
    source.read_to_end(&mut buffer).expect("Failed to read from file");
    let string = String::from_utf8_lossy(&buffer);
    string.chars().count()
}