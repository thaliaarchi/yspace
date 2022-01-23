// Copyright (c) 2021 Andrew Archibald
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod bit_pack;
mod syntax;
mod token;
use std::{env, fs};
use syntax::{Parser, Version};
use token::{Lexer, Mapping};

fn main() -> std::io::Result<()> {
    let filename = env::args_os().nth(1).expect("Usage: yspace <file>");
    let src = fs::read(filename)?;
    let mut p = Parser::new(Lexer::new(&src, Mapping::DEFAULT));
    if p.any(|inst| inst.version() == Version::WS0_3) {
        println!("0.3");
    } else {
        println!("0.2");
    }
    Ok(())
}
