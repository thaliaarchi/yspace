// Copyright (c) 2021 Andrew Archibald
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{char, error, fmt, str};
use Token::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Token {
    S,
    T,
    L,
}

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

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.i;
        while self.i < self.src.len() {
            // Lazily decode UTF-8
            let (ch, size) = bstr::decode_utf8(&self.src[self.i..]);
            self.i += size;
            if let Some(tok) = self.map.from_char(ch.expect("invalid UTF-8")) {
                let comment = &self.src[start..self.i - size];
                // SAFETY: already checked as UTF-8
                return Some((tok, unsafe { str::from_utf8_unchecked(comment) }));
            }
        }
        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mapping {
    s: char,
    t: char,
    l: char,
}

impl Mapping {
    #[inline]
    #[must_use]
    pub const fn new(s: char, t: char, l: char) -> Option<Self> {
        if s == t || t == l || s == l {
            return None;
        }
        Some(Mapping { s, t, l })
    }

    #[inline]
    #[must_use]
    pub const fn from_char(&self, ch: char) -> Option<Token> {
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
    #[must_use]
    pub const fn to_char(&self, tok: Token) -> char {
        match tok {
            S => self.s,
            T => self.t,
            L => self.l,
        }
    }
}

impl const Default for Mapping {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Mapping {
            s: ' ',
            t: '\t',
            l: '\n',
        }
    }
}

impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.s, self.t, self.l)
    }
}

impl str::FromStr for Mapping {
    type Err = MappingFromStrError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        use MappingFromStrError::*;
        let mut chars = v.chars();
        let s = chars.next().ok_or(NotThreeChars)?;
        let t = chars.next().ok_or(NotThreeChars)?;
        let l = chars.next().ok_or(NotThreeChars)?;
        if chars.as_str().len() != 0 {
            return Err(NotThreeChars);
        }
        Mapping::new(s, t, l).ok_or(EqualChars)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MappingFromStrError {
    EqualChars,
    NotThreeChars,
}

impl error::Error for MappingFromStrError {}

impl fmt::Display for MappingFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MappingFromStrError::*;
        match self {
            EqualChars => write!(f, "characters in mapping are not unique"),
            NotThreeChars => write!(f, "mapping is not exactly three characters"),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    // Annotated example from tutorial
    // https://web.archive.org/web/20150618184706/http://compsoc.dur.ac.uk/whitespace/tutorial.php
    pub const TUTORIAL_SRC: &'static [u8] = b"   \t\n\n   \t    \t\t\n \n \t\n \t   \t \t \n\t\n     \t\n\t    \n    \t \t\t\n\t  \t\n\t  \t   \t \t\n\n \n \t    \t\t\n\n   \t   \t \t\n \n\n\n\n\n";
    pub const TUTORIAL_TOKENS: [Token; 102] = [
        S, S, S, T, L, L, S, S, S, T, S, S, S, S, T, T, L, S, L, S, T, L, S, T, S, S, S, T, S, T,
        S, L, T, L, S, S, S, S, S, T, L, T, S, S, S, S, L, S, S, S, S, T, S, T, T, L, T, S, S, T,
        L, T, S, S, T, S, S, S, T, S, T, L, L, S, L, S, T, S, S, S, S, T, T, L, L, S, S, S, T, S,
        S, S, T, S, T, L, S, L, L, L, L, L,
    ];

    #[test]
    fn lex_tutorial() {
        let tokens = Lexer::new(&TUTORIAL_SRC, Mapping::default())
            .map(|(tok, comment)| {
                assert!(comment.len() == 0);
                tok
            })
            .collect::<Vec<_>>();
        assert_eq!(tokens, TUTORIAL_TOKENS);
    }
}
