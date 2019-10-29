use std::io::{self, Read};

fn main() {
    println!("{}", getWord());
}

fn getWord() -> io::Result<()> {

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

     handle.read_to_string(&mut buffer)?;
    Ok(())
}
