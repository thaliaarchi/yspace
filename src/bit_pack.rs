use crate::token::Token::{self, L, S, T};

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
                Some(true) => Some((L, "")),
                Some(false) => Some((T, "")),
                None => None, // marker bit
            },
            Some(false) => Some((S, "")),
            None => None,
        }
    }
}
