use self::Token::*;
use std::{char, env, fs};

#[derive(Debug, Clone)]
pub enum Token {
    S,
    T,
    L,
}

struct Lexer<'a> {
    src: &'a [u8],
    i: usize,
    map: Mapping,
}

impl<'a> Lexer<'a> {
    pub fn new<T: AsRef<[u8]>>(src: &'a T, map: Mapping) -> Self {
        Lexer {
            src: src.as_ref(),
            i: 0,
            map,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while self.i < self.src.len() {
            // Lazily decode UTF-8
            let (ch, size) = bstr::decode_utf8(&self.src[self.i..]);
            self.i += size;
            match self.map.from_char(ch.expect("invalid UTF-8")) {
                Some(tok) => return Some(tok),
                None => {}
            }
        }
        None
    }
}

struct Mapping {
    s: char,
    t: char,
    l: char,
}

impl Mapping {
    fn new(s: char, t: char, l: char) -> Self {
        Self { s, t, l }
    }

    fn from_char(&self, ch: char) -> Option<Token> {
        if ch == self.s {
            Some(S)
        } else if ch == self.t {
            Some(T)
        } else if ch == self.l {
            Some(L)
        } else {
            None
        }
    }

    fn to_char(&self, tok: Token) -> char {
        match tok {
            S => self.s,
            T => self.t,
            L => self.l,
        }
    }
}

fn main() -> std::io::Result<()> {
    let filename = env::args_os().nth(1).expect("Usage: wspace <file>");
    let src = fs::read(filename)?;
    let l = Lexer::new(&src, Mapping::new(' ', '\t', '\n'));
    let stl = Mapping::new('S', 'T', 'L');
    l.for_each(|tok| print!("{}", stl.to_char(tok)));
    Ok(())
}
