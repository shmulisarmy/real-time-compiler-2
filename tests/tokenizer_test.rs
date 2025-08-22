use compiler_11::lexer::token::{Token, TokenType};
use compiler_11::lexer::tokenizer::Tokenizer;

fn collect_types_and_values(mut t: Tokenizer<'_>) -> Vec<(TokenType, String)> {
    let mut out = Vec::new();
    while let Some(tok) = t.next() {
        out.push((tok.type_.clone(), tok.value.clone()))
    }
    out
}

#[test]
fn tokenize_identifiers_numbers_and_operators() {
    let src = "a + b*7".to_string();
    let t = Tokenizer::new(&src);
    let toks = collect_types_and_values(t);
    let expected = vec![
        (TokenType::Identifier, "a".into()),
        (TokenType::Operator, "+".into()),
        (TokenType::Identifier, "b".into()),
        (TokenType::Operator, "*".into()),
        (TokenType::Number, "7".into()),
    ];
    assert_eq!(toks, expected);
}

#[test]
fn tokenize_strings_and_punctuation() {
    let src = "(\"hi\", 42)".to_string();
    let t = Tokenizer::new(&src);
    let toks = collect_types_and_values(t);
    let expected = vec![
        (TokenType::Punctuation, "(".into()),
        (TokenType::String, "hi".into()),
        (TokenType::Punctuation, ",".into()),
        (TokenType::Number, "42".into()),
        (TokenType::Punctuation, ")".into()),
    ];
    assert_eq!(toks, expected);
}

#[test]
fn tokenize_keywords_vs_identifiers() {
    let src = "var x = func".to_string();
    let mut t = Tokenizer::new(&src);


    let expected = vec![
        (TokenType::Keyword, "var".into()),
        (TokenType::Identifier, "x".into()),
        (TokenType::Operator, "=".into()),
        (TokenType::Keyword, "func".into()),
    ];

    let toks = collect_types_and_values(t);
    assert_eq!(toks, expected);
}

#[test]
fn peek_does_not_advance() {
    let src = "x,y".to_string();
    let mut t = Tokenizer::new(&src);
    let expected = vec![
        (TokenType::Identifier, "x".into()),
        (TokenType::Punctuation, ",".into()),
        (TokenType::Identifier, "y".into()),
    ];
    let toks = collect_types_and_values(t);
    assert_eq!(toks, expected);
}
