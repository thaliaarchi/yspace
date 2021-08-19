// Copyright (c) 2021 Andrew Archibald
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::token::{Lexer, L, S, T};
pub use rug::Integer as Int;
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

pub struct Label {
    buf: Vec<u8>,
    bits: u8,
}

pub struct Parser<'a> {
    lex: Lexer<'a>,
}

impl Iterator for Parser<'_> {
    type Item = Inst;

    fn next(&mut self) -> Option<Inst> {
        match self.lex.next()? {
            // Stack manipulation
            S => match self.lex.next()? {
                S => Some(Push(self.parse_int())),
                T => match self.lex.next()? {
                    S => Some(Copy(self.parse_int())),
                    T => None,
                    L => Some(Slide(self.parse_int())),
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
                    S => Some(Label(self.parse_label())),
                    T => Some(Call(self.parse_label())),
                    L => Some(Jmp(self.parse_label())),
                },
                T => match self.lex.next()? {
                    S => Some(Jz(self.parse_label())),
                    T => Some(Jn(self.parse_label())),
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
    fn parse_int(&mut self) -> Int {
        Int::from(0)
    }

    fn parse_label(&mut self) -> Label {
        Label {
            buf: vec![],
            bits: 0,
        }
    }
}
