use std::{char, env, fs, str};
pub use Token::{L, S, T};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    S,
    T,
    L,
}

const EMPTY_COMMENT: &str = "";

pub struct Lexer<'a> {
    src: &'a [u8],
    i: usize,
    map: Mapping,
}

impl<'a> Lexer<'a> {
    #[inline]
    pub fn new<B: AsRef<[u8]>>(src: &'a B, map: Mapping) -> Self {
        Lexer {
            src: src.as_ref(),
            i: 0,
            map,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, &'a str);

    fn next(&mut self) -> Option<(Token, &'a str)> {
        let start = self.i;
        while self.i < self.src.len() {
            // Lazily decode UTF-8
            let (ch, size) = bstr::decode_utf8(&self.src[self.i..]);
            self.i += size;
            if let Some(tok) = self.map.from_char(ch.expect("invalid UTF-8")) {
                let comment = &self.src[start..self.i - size];
                return Some((tok, unsafe { str::from_utf8_unchecked(comment) }));
            }
        }
        None
    }
}

pub struct BitLexer<'a> {
    src: &'a [u8],
    i: usize,
    bit: u8,
}

impl<'a> BitLexer<'a> {
    #[inline]
    pub fn new<B: AsRef<[u8]>>(src: &'a B) -> Self {
        BitLexer {
            src: src.as_ref(),
            i: 0,
            bit: 7,
        }
    }

    pub fn next_bit(&mut self) -> Option<bool> {
        if self.i >= self.src.len() {
            return None;
        }
        let b = self.src[self.i];
        // Ignore trailing zeros on the last byte
        if self.i + 1 == self.src.len() && b << (7 - self.bit) == 0 {
            return None;
        }
        let bit = b & (1 << self.bit) != 0;
        if self.bit == 0 {
            self.bit = 7;
            self.i += 1;
        } else {
            self.bit -= 1;
        }
        Some(bit)
    }
}

impl<'a> Iterator for BitLexer<'a> {
    type Item = (Token, &'a str);

    #[inline]
    fn next(&mut self) -> Option<(Token, &'a str)> {
        match self.next_bit() {
            Some(true) => match self.next_bit() {
                Some(true) => Some((L, EMPTY_COMMENT)),
                Some(false) => Some((T, EMPTY_COMMENT)),
                None => None, // marker bit
            },
            Some(false) => Some((S, EMPTY_COMMENT)),
            None => None,
        }
    }
}

pub const DEFAULT_MAPPING: Mapping = Mapping {
    s: ' ',
    t: '\t',
    l: '\n',
};
pub const STL_MAPPING: Mapping = Mapping {
    s: 'S',
    t: 'T',
    l: 'L',
};

pub struct Mapping {
    s: char,
    t: char,
    l: char,
}

impl Mapping {
    #[inline]
    pub fn new(s: char, t: char, l: char) -> Self {
        Mapping { s, t, l }
    }

    #[inline]
    pub fn from_char(&self, ch: char) -> Option<Token> {
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

    #[inline]
    pub fn to_char(&self, tok: &Token) -> char {
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
    let l = Lexer::new(&src, DEFAULT_MAPPING);
    l.for_each(|(tok, comment)| println!("{}:{}", STL_MAPPING.to_char(&tok), comment));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TUTORIAL_TOKENS: [Token; 102] = [
        S, S, S, T, L, L, S, S, S, T, S, S, S, S, T, T, L, S, L, S, T, L, S, T, S, S, S, T, S, T,
        S, L, T, L, S, S, S, S, S, T, L, T, S, S, S, S, L, S, S, S, S, T, S, T, T, L, T, S, S, T,
        L, T, S, S, T, S, S, S, T, S, T, L, L, S, L, S, T, S, S, S, S, T, T, L, L, S, S, S, T, S,
        S, S, T, S, T, L, S, L, L, L, L, L,
    ];

    #[test]
    fn lex_tutorial() {
        // Annotated example from tutorial
        // https://web.archive.org/web/20150618184706/http://compsoc.dur.ac.uk/whitespace/tutorial.php
        let src = b"   \t\n\n   \t    \t\t\n \n \t\n \t   \t \t \n\t\n     \t\n\t    \n    \t \t\t\n\t  \t\n\t  \t   \t \t\n\n \n \t    \t\t\n\n   \t   \t \t\n \n\n\n\n\n";
        let tokens = Lexer::new(&src, DEFAULT_MAPPING)
            .map(|(tok, comment)| {
                assert!(comment.len() == 0);
                tok
            })
            .collect::<Vec<Token>>();
        assert!(tokens.len() == TUTORIAL_TOKENS.len());
        assert!(tokens.iter().zip(&TUTORIAL_TOKENS).all(|(a, b)| a == b));
    }

    #[test]
    fn bit_lex_tutorial() {
        let src = [
            0b00010111, 0b10001000, 0b00101011, 0b01101011, 0b01000010, 0b01001110, 0b11000001,
            0b01110000, 0b01100001, 0b00101011, 0b10001011, 0b10001000, 0b01001011, 0b11011010,
            0b00001010, 0b11110001, 0b00001001, 0b01101111, 0b11111100,
        ];
        let tokens = BitLexer::new(&src)
            .map(|(tok, comment)| {
                assert!(comment.len() == 0);
                tok
            })
            .collect::<Vec<Token>>();
        assert!(tokens.len() == TUTORIAL_TOKENS.len());
        assert!(tokens.iter().zip(&TUTORIAL_TOKENS).all(|(a, b)| a == b));
    }
}
