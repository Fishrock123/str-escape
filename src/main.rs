use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = vec![];
    let mut stdin = io::stdin();
    stdin.read_to_end(&mut buffer)?;

    let input = String::from_utf8(buffer).unwrap();
    let output: String = input.escape_default().collect();

    print!("{output}");

    Ok(())
}
