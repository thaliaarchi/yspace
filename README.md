# yspace

yspace is a work-in-progress toolchain for the Whitespace programming
language written in Rust.

## Planned features

- wspls language server
  - Inline values for instruction names that are preserved while editing
  - Debugging
- wsvsc VS Code extension
  - Syntax highlighting
  - Render whitespace characters
  - Disassembly panel
- wsasm assembler
  - Support all dialects
  - Macros like [Whitelips](https://vii5ard.github.io/whitespace/) and
    others
  - Support constant or address arguments like
    [WhitespaceAssembler](https://github.com/littleBugHunter/WhitespaceAssembler)
  - Linting
    - Warn on usage of multiple mnemonic for same instruction
    - Warn on inconsistent mnemonics between corresponding IO
      instructions
    - Warn on inconsistent mnemonic capitalization
- wsdisasm disassembler
  - Format according to dialect definition
  - Automatically collapse constant or address arguments
  - Format constants in ASCII range as chars
- wspack compresser
  - wsx-format compression and decompression
- wspace interpreter
  - File execution
  - Whitespace Assembly REPL

## Languages

- Stack-based like Factor
  - First-class closures (see
    [“Closure elimination as constant propagation”](https://web.archive.org/web/20110726044425/http://factorcode.org/littledan/abstract.pdf))

## Architecture plans

- Written in Rust
- Parses with parser combinators
- LLVM backend
- Use e-graphs for optimization ordering
  ([Egg](https://egraphs-good.github.io/) library)

## License

This project is made available under the terms of the
[Mozilla Public License, v. 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
