// Copyright (c) 2021 Andrew Archibald
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::token::{
    Lexer,
    Token::{self, L, S, T},
};
use rug::{integer::Order, ops::NegAssign, Integer};
use std::{fmt, str};
pub use Inst::*;

pub enum Inst {
    Push(Int),
    Dup,
    Copy(Int),
    Swap,
    Drop,
    Slide(Int),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Store,
    Retrieve,
    Label(Label),
    Call(Label),
    Jmp(Label),
    Jz(Label),
    Jn(Label),
    Ret,
    End,
    Printc,
    Printi,
    Readc,
    Readi,
}

impl Inst {
    pub fn arg(&self) -> Option<&Int> {
        match self {
            Push(i) => Some(i),
            Copy(i) => Some(i),
            Slide(i) => Some(i),
            _ => None,
        }
    }

    pub fn label(&self) -> Option<&Label> {
        match self {
            Label(l) => Some(l),
            Call(l) => Some(l),
            Jmp(l) => Some(l),
            Jz(l) => Some(l),
            Jn(l) => Some(l),
            _ => None,
        }
    }

    pub fn to_tokens(&self, v: &mut Vec<Token>) {
        v.extend_from_slice(self.ws_opcode());
        if let Some(n) = self.arg() {
            n.to_tokens(v);
        } else if let Some(l) = self.label() {
            l.to_tokens(v);
        }
    }

    pub fn ws_opcode(&self) -> &'static [Token] {
        match self {
            Push(_) => &[S, S],
            Dup => &[S, L, S],
            Copy(_) => &[S, T, S],
            Swap => &[S, L, T],
            Drop => &[S, L, L],
            Slide(_) => &[S, T, L],
            Add => &[T, S, S, S],
            Sub => &[T, S, S, T],
            Mul => &[T, S, S, L],
            Div => &[T, S, T, S],
            Mod => &[T, S, T, T],
            Store => &[T, T, S],
            Retrieve => &[T, T, T],
            Label(_) => &[L, S, S],
            Call(_) => &[L, S, T],
            Jmp(_) => &[L, S, L],
            Jz(_) => &[L, T, S],
            Jn(_) => &[L, T, T],
            Ret => &[L, T, L],
            End => &[L, L, L],
            Printc => &[T, L, S, S],
            Printi => &[T, L, S, T],
            Readc => &[T, L, T, S],
            Readi => &[T, L, T, T],
        }
    }

    pub fn wsa_opcode(&self) -> &'static str {
        match self {
            Push(_) => "push",
            Dup => "dup",
            Copy(_) => "copy",
            Swap => "swap",
            Drop => "drop",
            Slide(_) => "slide",
            Add => "add",
            Sub => "sub",
            Mul => "mul",
            Div => "div",
            Mod => "mod",
            Store => "store",
            Retrieve => "retrieve",
            Label(_) => "label",
            Call(_) => "call",
            Jmp(_) => "jmp",
            Jz(_) => "jz",
            Jn(_) => "jn",
            Ret => "ret",
            End => "end",
            Printc => "printc",
            Printi => "printi",
            Readc => "readc",
            Readi => "readi",
        }
    }

    pub fn version(&self) -> Version {
        match self {
            Copy(_) | Slide(_) => Version::WS0_3,
            _ => Version::WS0_2,
        }
    }
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Label(l) = self {
            write!(f, "{}:", l.val)
        } else if let Some(n) = self.arg() {
            write!(f, "{} {}", self.wsa_opcode(), n.val)
        } else if let Some(l) = self.label() {
            write!(f, "{} {}", self.wsa_opcode(), l.val)
        } else {
            write!(f, "{}", self.wsa_opcode())
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Version {
    WS0_2,
    WS0_3,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Version::WS0_2 => write!(f, "0.2"),
            Version::WS0_3 => write!(f, "0.3"),
        }
    }
}

pub struct Int {
    val: Integer,
    raw: RawUint,
    sign: Sign,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Sign {
    Pos,
    Neg,
    Empty,
}

impl Int {
    fn empty() -> Self {
        Int {
            val: Integer::new(),
            raw: RawUint::new(),
            sign: Sign::Empty,
        }
    }

    pub fn to_tokens(&self, v: &mut Vec<Token>) {
        match self.sign {
            Sign::Pos => v.push(S),
            Sign::Neg => v.push(T),
            Sign::Empty => {}
        }
        self.raw.to_tokens(v);
    }
}

pub struct Label {
    val: Integer,
    raw: RawUint,
}

impl Label {
    pub fn as_utf8(&self) -> Option<&str> {
        if self.raw.len() % 8 == 0 {
            if let Ok(s) = str::from_utf8(&self.raw.buf) {
                return Some(s);
            }
        }
        None
    }

    pub fn to_tokens(&self, v: &mut Vec<Token>) {
        self.raw.to_tokens(v);
    }
}

#[derive(Clone)]
struct RawUint {
    buf: Vec<u8>,
    len: usize,
}

impl RawUint {
    fn new() -> Self {
        RawUint {
            buf: Vec::new(),
            len: 0,
        }
    }

    fn from_tokens(toks: &Vec<Token>) -> Self {
        let len = (toks.len() + 7) / 8;
        let mut buf = vec![0; len];
        let (mut i, mut bit) = (len, 7);
        for tok in toks.iter().rev() {
            if bit == 7 {
                i -= 1;
                bit = 0;
            } else {
                bit += 1;
            }
            if *tok == T {
                buf[i] |= 1 << bit;
            }
        }
        RawUint {
            buf,
            len: toks.len(),
        }
    }

    fn to_tokens(&self, toks: &mut Vec<Token>) {
        toks.reserve(self.len + 1);
        for i in 0..self.len {
            toks.push(if self.bit(i) { T } else { S })
        }
        toks.push(L)
    }

    fn bit(&self, i: usize) -> bool {
        (self.buf[i / 8] >> (7 - i % 8)) & 1 == 1
    }

    fn leading_zeros(&self) -> usize {
        for (i, b) in self.buf.iter().enumerate() {
            if *b != 0 {
                return i * 8 + b.leading_zeros() as usize - self.len % 8;
            }
        }
        self.len
    }

    fn significant_bits(&self) -> usize {
        self.len() - self.leading_zeros()
    }

    fn to_integer(&self) -> Integer {
        Integer::from_digits(&self.buf, Order::MsfBe)
    }

    fn len(&self) -> usize {
        self.len
    }
}

pub struct Parser<'a> {
    lex: &'a mut Lexer<'a>,
    tok_buf: Vec<Token>,
    toks: usize,
    comments: Option<Vec<String>>,
}

impl<'a> Parser<'a> {
    pub fn new(lex: &'a mut Lexer<'a>) -> Self {
        Parser {
            lex,
            tok_buf: vec![],
            toks: 0,
            comments: None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let (tok, comment) = self.lex.next()?;
        if comment.len() != 0 {
            if self.comments == None {
                self.comments = Some(vec![String::new(); self.toks]);
            }
            self.comments.as_mut().unwrap().push(comment.to_string());
        }
        Some(tok)
    }
}

impl Iterator for Parser<'_> {
    type Item = Inst;

    fn next(&mut self) -> Option<Inst> {
        self.toks = 0;
        match self.next_token()? {
            // Stack manipulation
            S => match self.next_token()? {
                S => Some(Push(self.parse_int()?)),
                T => match self.next_token()? {
                    S => Some(Copy(self.parse_int()?)),
                    T => None,
                    L => Some(Slide(self.parse_int()?)),
                },
                L => match self.next_token()? {
                    S => Some(Dup),
                    T => Some(Swap),
                    L => Some(Drop),
                },
            },

            T => match self.next_token()? {
                // Arithmetic
                S => match self.next_token()? {
                    S => match self.next_token()? {
                        S => Some(Add),
                        T => Some(Sub),
                        L => Some(Mul),
                    },
                    T => match self.next_token()? {
                        S => Some(Div),
                        T => Some(Mod),
                        L => None,
                    },
                    L => None,
                },

                // Heap access
                T => match self.next_token()? {
                    S => Some(Store),
                    T => Some(Retrieve),
                    L => None,
                },

                // I/O
                L => match self.next_token()? {
                    S => match self.next_token()? {
                        S => Some(Printc),
                        T => Some(Printi),
                        L => None,
                    },
                    T => match self.next_token()? {
                        S => Some(Readc),
                        T => Some(Readi),
                        L => None,
                    },
                    L => None,
                },
            },

            // Control flow
            L => match self.next_token()? {
                S => match self.next_token()? {
                    S => Some(Label(self.parse_label()?)),
                    T => Some(Call(self.parse_label()?)),
                    L => Some(Jmp(self.parse_label()?)),
                },
                T => match self.next_token()? {
                    S => Some(Jz(self.parse_label()?)),
                    T => Some(Jn(self.parse_label()?)),
                    L => Some(Ret),
                },
                L => match self.next_token()? {
                    L => Some(End),
                    _ => None,
                },
            },
        }
    }
}

impl Parser<'_> {
    fn parse_uint(&mut self) -> Option<RawUint> {
        self.tok_buf.clear();
        loop {
            let tok = self.next_token()?;
            if tok == L {
                break;
            }
            self.tok_buf.push(tok);
        }
        Some(RawUint::from_tokens(&self.tok_buf))
    }

    fn parse_int(&mut self) -> Option<Int> {
        let sign = match self.next_token()? {
            S => Sign::Pos,
            T => Sign::Neg,
            L => return Some(Int::empty()),
        };
        let raw = self.parse_uint()?;
        let mut val = raw.to_integer();
        if sign == Sign::Neg {
            val.neg_assign();
        }
        Some(Int { val, raw, sign })
    }

    fn parse_label(&mut self) -> Option<Label> {
        let raw = self.parse_uint()?;
        let val = raw.to_integer();
        Some(Label { val, raw })
    }
}
