#[derive(Copy, Clone)]
enum ParserState {
    WordBegin,
    InsideWord,
}

fn count() -> Result<u32, std::io::Error> {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    
    let mut count = 0;

    for line in stdin.lines() {
        let mut state = ParserState::WordBegin;
        for c in line?.chars() {
            match (state, c) {
                (ParserState::WordBegin, 'l') => {
                    count += 1;
                    state = ParserState::InsideWord;
                },
                (_, ' ') => state = ParserState::WordBegin,
                (_, '\t') => state = ParserState::WordBegin,
                _ => state = ParserState::InsideWord,
            }
        }
    }
    Ok(count)
}

fn main() {
    match count() {
        Ok(cnt) => println!("Count: {}", cnt),
        Err(e) => println!("Error: {}", e),
    }
}
