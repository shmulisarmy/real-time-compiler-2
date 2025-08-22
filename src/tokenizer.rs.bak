use crate::token::{Token, TokenType};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static OPERATOR_CHARS: &str = "+-*/|=<>";
static PUNCTUATION_CHARS: &str = "()[]{},:";

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut hm = HashMap::new();
    hm.insert("var", TokenType::Keyword);
    hm.insert("return", TokenType::Keyword);
    hm.insert("if", TokenType::Keyword);
    hm.insert("else", TokenType::Keyword);
    hm.insert("while", TokenType::Keyword);
    hm.insert("for", TokenType::Keyword);
    hm.insert("break", TokenType::Keyword);
    hm.insert("continue", TokenType::Keyword);
    hm.insert("func", TokenType::Keyword);
    hm.insert("struct", TokenType::Keyword);
    hm
});

pub struct Tokenizer {
    pub source: String,
    pub index: usize,
}

impl Tokenizer {
    pub fn new(source: String) -> Tokenizer {
        Tokenizer { source, index: 0 }
    }

    fn cur_char(&self) -> char {
        self.source.chars().nth(self.index).unwrap()
    }
    fn next_word(&mut self) -> String {
        let start = self.index;
        while self.in_range()
            && ((self.cur_char() >= 'a' && self.cur_char() <= 'z')
                || (self.cur_char() >= 'A' && self.cur_char() <= 'Z')
                || self.cur_char() == '_')
        {
            self.index += 1;
        }
        self.source[start..self.index].to_string()
    }

    fn next_number(&mut self) -> String {
        let start = self.index;
        while self.in_range() && (self.cur_char().is_numeric() || self.cur_char() == '.') {
            self.index += 1;
        }
        self.source[start..self.index].to_string()
    }

    fn next_string(&mut self) -> String {
        self.index += 1;
        let start = self.index;
        while self.cur_char() != '"' {
            self.index += 1;
        }
        let end = self.index;
        self.index += 1;
        self.source[start..end].to_string()
    }
    pub fn in_range(&self) -> bool {
        self.index < self.source.len()
    }
    pub fn eat_lines(&mut self) {
        while self.cur_char() == ' ' || self.cur_char() == '\t' || self.cur_char() == '\n' {
            self.index += 1;
        }
    }
    fn eat_spaces(&mut self) {
        while self.in_range() && (self.cur_char() == ' ' || self.cur_char() == '\t') {
            self.index += 1;
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        let index = self.index;
        let token = self.next();
        self.index = index;
        token
    }
    pub fn next(&mut self) -> Option<Token> {
        let next_token = self.private_next();
        // println!("just tokenized {:?}", next_token);
        next_token
    }

    fn next_operator(&mut self) -> String {
        let start = self.index;
        while self.in_range() && OPERATOR_CHARS.contains(self.cur_char()) {
            self.index += 1;
        }
        let end = self.index;
        self.source[start..end].to_string()
    }

    fn private_next(&mut self) -> Option<Token> {
        println!(
            "[tokenizer] next_under_wapper() called, current index: {}",
            self.index
        );
        self.eat_spaces();

        if !self.in_range() {
            println!("[tokenizer] No more characters in range, returning None");
            return None;
        }

        let current_char = self.cur_char();
        println!(
            "[tokenizer] Current character: '{}' (index: {})",
            current_char, self.index
        );

        if current_char.is_alphabetic() {
            println!("[tokenizer] Found alphabetic character, reading word...");
            let word = self.next_word();
            println!("[tokenizer] Read word: '{}'", word);

            if let Some(keyword) = KEYWORDS.get(word.as_str()) {
                let token = Token {
                    type_: keyword.clone(),
                    value: word,
                };
                println!("[tokenizer] Matched keyword: {:?}", token);
                return Some(token);
            }

            let token = Token {
                type_: TokenType::Identifier,
                value: word,
            };
            println!("[tokenizer] Created identifier token: {:?}", token);
            return Some(token);
        }

        if current_char.is_numeric() {
            println!("[tokenizer] Found numeric character, reading number...");
            let number = self.next_number();
            let token = Token {
                type_: TokenType::Number,
                value: number,
            };
            println!("[tokenizer] Created number token: {:?}", token);
            return Some(token);
        }

        if current_char == '"' {
            println!("[tokenizer] Found string delimiter, reading string...");
            let string_val = self.next_string();
            let token = Token {
                type_: TokenType::String,
                value: string_val,
            };
            println!("[tokenizer] Created string token: {:?}", token);
            return Some(token);
        }

        if PUNCTUATION_CHARS.contains(current_char) {
            let this_char = current_char;
            println!("[tokenizer] Found punctuation: '{}'", this_char);
            self.index += 1;
            let token = Token {
                type_: TokenType::Punctuation,
                value: this_char.to_string(),
            };
            println!("[tokenizer] Created punctuation token: {:?}", token);
            return Some(token);
        }

        if OPERATOR_CHARS.contains(current_char) {
            let operator = self.next_operator();
            let token = Token {
                type_: TokenType::Operator,
                value: operator,
            };
            println!("[tokenizer] Created operator token: {:?}", token);
            return Some(token);
        }

        match current_char {
            ' ' => {
                println!("[tokenizer] Found space, skipping and recursing...");
                self.index += 1;
                self.next()
            }
            _ => {
                println!(
                    "[tokenizer] Unrecognized character: '{}' (index: {})",
                    current_char, self.index
                );
                None
            }
        }
    }

    pub fn get_line_and_col_from_pos(&self, pos: usize) -> (usize, usize) {
        let line = self.source.lines().count();
        let col = self.source.lines().nth(line - 1).unwrap().len();
        (line, col)
    }

    pub fn show_user_error(&mut self, start_pos: usize, end_pos: usize, message: String) {
        let all_text_up_until_this_point = &self.source[..start_pos];
        print!("{}", all_text_up_until_this_point);
        println!("{}", red(self.source[start_pos..end_pos].to_string()));
        panic!(
            "{} on line {} column {}",
            red(message),
            self.get_line_and_col_from_pos(start_pos).0,
            self.get_line_and_col_from_pos(start_pos).1
        );
    }
    pub fn expect(&mut self, type_: TokenType) -> Token {
        let start_pos = self.index;
        let token = self.next().unwrap();
        if token.type_ != type_ {
            self.show_user_error(
                start_pos,
                self.index,
                format!("Expected {:?} got {:?}", type_, token.type_),
            );
        }
        token
    }

    pub fn expect_punctuation(&mut self, value: char) -> Token {
        let token = self.next().unwrap();
        if token.type_ != TokenType::Punctuation
            || token.value.len() != 1
            || token.value.chars().nth(0).unwrap() != value
        {
            self.show_user_error(
                self.index,
                self.index,
                format!("Expected {} got {}", value, token.value),
            );
        }
        token
    }
    fn optionally_expect(&mut self, type_: TokenType) -> Option<String> {
        let position_at_start = self.index;
        let token = self.next();
        if token.is_none() {
            self.index = position_at_start;
            return None;
        }
        let token = token.unwrap();
        if token.type_ != type_ {
            self.index = position_at_start;
            return None;
        }
        Some(token.value)
    }

    pub fn optionally_expect_string(&mut self, value: &str) -> bool {
        if !self.in_range() {
            return false;
        }
        let position_at_start = self.index;
        let token = self.next();
        if token.is_none() {
            self.index = position_at_start;
            return false;
        }
        let token = token.unwrap();
        if token.value != value {
            self.index = position_at_start;
            return false;
        }
        true
    }

    pub fn optionally_expect_keyword_of(&mut self, value: &str) -> bool {
        let position_at_start = self.index;
        let token = self.next();
        if token.is_none() {
            self.index = position_at_start;
            return false;
        }
        let token = token.unwrap();
        if token.type_ != TokenType::Keyword {
            self.index = position_at_start;
            return false;
        }
        if token.value == value {
            return true;
        }
        self.index = position_at_start;
        return false;
    }

    pub fn optionally_expect_punctuation(&mut self, value: char) -> bool {
        let position_at_start = self.index;
        let token = self.next();
        if token.is_none() {
            self.index = position_at_start;
            return false;
        }
        let token = token.unwrap();
        if token.type_ != TokenType::Punctuation {
            self.index = position_at_start;
            return false;
        }
        println!(
            "in optionally_expect_punctuation: token.value.chars().nth(0).unwrap(): {}",
            token.value.chars().nth(0).unwrap()
        );
        if token.value.chars().nth(0).unwrap() == value {
            println!("returning true");
            return true;
        }
        println!("returning false");
        self.index = position_at_start;
        return false;
    }
}

fn red(text: String) -> String {
    format!("\x1B[31m{}\x1B[0m", text)
}
