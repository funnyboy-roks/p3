use std::{
    fs,
    io::{self, BufWriter},
};

use lexer::Lexer;

mod lexer;

fn main() -> io::Result<()> {
    let file = std::env::args().nth(1).expect("Usage: p3 <file> <out>");
    let out_path = std::env::args().nth(2).expect("Usage: p3 <file> <out>");
    let out = fs::File::create(out_path)?;
    let mut out = BufWriter::new(out);
    println!("reading");
    for tok in Lexer::new(fs::read_to_string(file)?.chars().collect()) {
        println!("{:?}", tok);
        tok.write_to(&mut out)?;
    }
    Ok(())
}
