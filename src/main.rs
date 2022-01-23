// Copyright (c) 2021 Andrew Archibald
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(const_option)]
#![feature(const_option_ext)]
#![feature(const_trait_impl)]

mod bit_pack;
mod syntax;
mod token;

use clap::{ArgEnum, Parser as ClapParser};
use std::{fs, path::PathBuf};
use syntax::{Parser, Version};
use token::{Lexer, Mapping};

#[derive(ClapParser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Subcommand to execute
    #[clap(arg_enum)]
    command: Command,
    /// Filename of Whitespace program
    file: PathBuf,
    /// Token mapping
    #[clap(short, long, default_value_t)]
    mapping: Mapping,
}

#[derive(Copy, Clone, PartialEq, Eq, ArgEnum)]
enum Command {
    /// Disassemble program
    Disasm,
    /// Display Whitespace specification required by program
    Spec,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let src = fs::read(cli.file)?;
    let mut p = Parser::new(Lexer::new(&src, cli.mapping));
    match cli.command {
        Command::Disasm => p.for_each(|inst| println!("{}", inst)),
        Command::Spec => {
            if p.any(|inst| inst.version() == Version::WS0_3) {
                println!("0.3");
            } else {
                println!("0.2");
            }
        }
    }
    Ok(())
}

#[test]
fn verify_app() {
    use clap::IntoApp;
    Cli::into_app().debug_assert();
}
