// Copyright (c) 2021 Andrew Archibald
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod bit_pack;
mod syntax;
mod token;
use std::{env, fs};
use syntax::Parser;
use token::{Lexer, Mapping};

fn main() -> std::io::Result<()> {
    let filename = env::args_os().nth(1).expect("Usage: yspace <file>");
    let src = fs::read(filename)?;
    let mut lex = Lexer::new(&src, Mapping::DEFAULT);
    let p = Parser::new(&mut lex);
    let mut v = Vec::new();
    p.for_each(|inst| inst.to_tokens(&mut v));
    v.into_iter()
        .for_each(|tok| print!("{}", Mapping::DEFAULT.to_char(tok)));
    Ok(())
}
