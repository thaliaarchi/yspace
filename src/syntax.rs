// Copyright (c) 2021 Andrew Archibald
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::token::{
    Lexer,
    Token::{self, L, S, T},
};
pub use rug::{integer::Order, Integer};
use std::fmt;
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

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Push(i) => write!(f, "push {}", i.val),
            Dup => write!(f, "dup"),
            Copy(i) => write!(f, "copy {}", i.val),
            Swap => write!(f, "swap"),
            Drop => write!(f, "drop"),
            Slide(i) => write!(f, "slide {}", i.val),
            Add => write!(f, "add"),
            Sub => write!(f, "sub"),
            Mul => write!(f, "mul"),
            Div => write!(f, "div"),
            Mod => write!(f, "mod"),
            Store => write!(f, "store"),
            Retrieve => write!(f, "retrieve"),
            Label(l) => write!(f, "label {}", l.val),
            Call(l) => write!(f, "call {}", l.val),
            Jmp(l) => write!(f, "jmp {}", l.val),
            Jz(l) => write!(f, "jz {}", l.val),
            Jn(l) => write!(f, "jn {}", l.val),
            Ret => write!(f, "ret"),
            End => write!(f, "end"),
            Printc => write!(f, "printc"),
            Printi => write!(f, "printi"),
            Readc => write!(f, "readc"),
            Readi => write!(f, "readi"),
        }
    }
}

pub struct Int {
    val: Integer,
    raw: RawUint,
    sign: Sign,
}

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
}

pub struct Label {
    val: Integer,
    raw: RawUint,
}

struct RawUint {
    buf: Vec<u8>,
    first_bits: u8,
}

impl RawUint {
    fn new() -> Self {
        RawUint {
            buf: Vec::new(),
            first_bits: 0,
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
            first_bits: bit,
        }
    }

    fn leading_zeros(&self) -> usize {
        if self.buf.len() == 0 {
            return 0;
        }
        if self.buf[0] != 0 {
            return self.buf[0].leading_zeros() as usize - (8 - self.first_bits as usize);
        }
        let mut leading_zeros = self.first_bits as usize;
        for b in &self.buf[1..] {
            if *b != 0 {
                return leading_zeros + b.leading_zeros() as usize;
            }
            leading_zeros += 8;
        }
        leading_zeros
    }

    fn significant_zeros(&self) -> usize {
        self.len() - self.leading_zeros()
    }

    fn as_integer(&self) -> Integer {
        if self.buf.len() == 0 {
            Integer::new()
        } else {
            Integer::from_digits(&self.buf, Order::MsfBe) // Order::LsfBe
        }
    }

    fn len(&self) -> usize {
        self.buf.len() * 8 + self.first_bits as usize
    }
}

pub struct Parser<'a> {
    lex: &'a mut Lexer<'a>,
    toks: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lex: &'a mut Lexer<'a>) -> Self {
        Parser { lex, toks: vec![] }
    }
}

impl Iterator for Parser<'_> {
    type Item = Inst;

    fn next(&mut self) -> Option<Inst> {
        match self.lex.next()? {
            // Stack manipulation
            S => match self.lex.next()? {
                S => Some(Push(self.parse_int()?)),
                T => match self.lex.next()? {
                    S => Some(Copy(self.parse_int()?)),
                    T => None,
                    L => Some(Slide(self.parse_int()?)),
                },
                L => match self.lex.next()? {
                    S => Some(Dup),
                    T => Some(Swap),
                    L => Some(Drop),
                },
            },

            T => match self.lex.next()? {
                // Arithmetic
                S => match self.lex.next()? {
                    S => match self.lex.next()? {
                        S => Some(Add),
                        T => Some(Sub),
                        L => Some(Mul),
                    },
                    T => match self.lex.next()? {
                        S => Some(Div),
                        T => Some(Mod),
                        L => None,
                    },
                    L => None,
                },

                // Heap access
                T => match self.lex.next()? {
                    S => Some(Store),
                    T => Some(Retrieve),
                    L => None,
                },

                // I/O
                L => match self.lex.next()? {
                    S => match self.lex.next()? {
                        S => Some(Printc),
                        T => Some(Printi),
                        L => None,
                    },
                    T => match self.lex.next()? {
                        S => Some(Readc),
                        T => Some(Readi),
                        L => None,
                    },
                    L => None,
                },
            },

            // Control flow
            L => match self.lex.next()? {
                S => match self.lex.next()? {
                    S => Some(Label(self.parse_label()?)),
                    T => Some(Call(self.parse_label()?)),
                    L => Some(Jmp(self.parse_label()?)),
                },
                T => match self.lex.next()? {
                    S => Some(Jz(self.parse_label()?)),
                    T => Some(Jn(self.parse_label()?)),
                    L => Some(Ret),
                },
                L => match self.lex.next()? {
                    L => Some(End),
                    _ => None,
                },
            },
        }
    }
}

impl Parser<'_> {
    fn parse_uint(&mut self) -> Option<RawUint> {
        self.toks.clear();
        loop {
            let tok = self.lex.next()?;
            if tok == L {
                break;
            }
            self.toks.push(tok);
        }
        Some(RawUint::from_tokens(&self.toks))
    }

    fn parse_int(&mut self) -> Option<Int> {
        let sign = match self.lex.next()? {
            S => Sign::Pos,
            T => Sign::Neg,
            L => return Some(Int::empty()),
        };
        let raw = self.parse_uint()?;
        let val = raw.as_integer();
        Some(Int { val, raw, sign })
    }

    fn parse_label(&mut self) -> Option<Label> {
        let raw = self.parse_uint()?;
        let val = raw.as_integer();
        Some(Label { val, raw })
    }
}
