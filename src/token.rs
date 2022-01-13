// Copyright (c) 2021 Andrew Archibald
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{char, str};
pub use Token::{L, S, T};

#[derive(Debug, Copy, Clone, PartialEq)]
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

pub struct Mapping {
    s: char,
    t: char,
    l: char,
}

impl Mapping {
    pub const DEFAULT: Mapping = Mapping {
        s: ' ',
        t: '\t',
        l: '\n',
    };
    pub const STL: Mapping = Mapping {
        s: 'S',
        t: 'T',
        l: 'L',
    };

    #[inline]
    #[must_use]
    pub fn new(s: char, t: char, l: char) -> Self {
        Mapping { s, t, l }
    }

    #[inline]
    #[must_use]
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
    #[must_use]
    pub fn to_char(&self, tok: Token) -> char {
        match tok {
            S => self.s,
            T => self.t,
            L => self.l,
        }
    }
}
