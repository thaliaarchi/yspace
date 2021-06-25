# yspace

yspace is a work-in-progress toolchain for the Whitespace programming
language.

## Planned features

- Written in Rust
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
- Use e-graphs for optimization ordering
  ([Egg](https://egraphs-good.github.io/) library)
- Backends
  - Truffle
  - LLVM

## License

This project is made available under the
[Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
