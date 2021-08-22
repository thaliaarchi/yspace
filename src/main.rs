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
use token::Lexer;

fn main() -> std::io::Result<()> {
    let filename = env::args_os().nth(1).expect("Usage: wspace <file>");
    let src = fs::read(filename)?;
    let mut lex = Lexer::new(&src, token::DEFAULT);
    // l.for_each(|tok| print!("{}", token::STL.to_char(&tok)));
    let p = Parser::new(&mut lex);
    p.for_each(|inst| println!("{}", inst));
    Ok(())
}
