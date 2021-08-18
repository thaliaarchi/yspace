mod bit_pack;
mod token;
use std::{env, fs};
use token::Lexer;

fn main() -> std::io::Result<()> {
    let filename = env::args_os().nth(1).expect("Usage: wspace <file>");
    let src = fs::read(filename)?;
    let l = Lexer::new(&src, token::DEFAULT);
    l.for_each(|(tok, comment)| println!("{}:{}", token::STL.to_char(&tok), comment));
    Ok(())
}
