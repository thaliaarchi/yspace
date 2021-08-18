use bit_pack::BitLexer;
use token::{Lexer, Token, DEFAULT, L, S, T};

const TUTORIAL_TOKENS: [Token; 102] = [
    S, S, S, T, L, L, S, S, S, T, S, S, S, S, T, T, L, S, L, S, T, L, S, T, S, S, S, T, S, T, S, L,
    T, L, S, S, S, S, S, T, L, T, S, S, S, S, L, S, S, S, S, T, S, T, T, L, T, S, S, T, L, T, S, S,
    T, S, S, S, T, S, T, L, L, S, L, S, T, S, S, S, S, T, T, L, L, S, S, S, T, S, S, S, T, S, T, L,
    S, L, L, L, L, L,
];

#[test]
fn lex_tutorial() {
    // Annotated example from tutorial
    // https://web.archive.org/web/20150618184706/http://compsoc.dur.ac.uk/whitespace/tutorial.php
    let src = b"   \t\n\n   \t    \t\t\n \n \t\n \t   \t \t \n\t\n     \t\n\t    \n    \t \t\t\n\t  \t\n\t  \t   \t \t\n\n \n \t    \t\t\n\n   \t   \t \t\n \n\n\n\n\n";
    let tokens = Lexer::new(&src, DEFAULT)
        .map(|(tok, comment)| {
            assert!(comment.len() == 0);
            tok
        })
        .collect::<Vec<_>>();
    assert!(tokens.len() == TUTORIAL_TOKENS.len());
    assert!(tokens.iter().zip(&TUTORIAL_TOKENS).all(|(a, b)| a == b));
}

#[test]
fn bit_lex_tutorial() {
    let src = [
        0b00010111, 0b10001000, 0b00101011, 0b01101011, 0b01000010, 0b01001110, 0b11000001,
        0b01110000, 0b01100001, 0b00101011, 0b10001011, 0b10001000, 0b01001011, 0b11011010,
        0b00001010, 0b11110001, 0b00001001, 0b01101111, 0b11111100,
    ];
    let tokens = BitLexer::new(&src)
        .map(|(tok, comment)| {
            assert!(comment.len() == 0);
            tok
        })
        .collect::<Vec<_>>();
    assert!(tokens.len() == TUTORIAL_TOKENS.len());
    assert!(tokens.iter().zip(&TUTORIAL_TOKENS).all(|(a, b)| a == b));
}
