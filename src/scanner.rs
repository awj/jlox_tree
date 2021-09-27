use crate::{ExecutionError};
use crate::token::{self, Literal, Token, TokenType};

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    pub errors: Vec<ExecutionError>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new()
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut vec = Vec::new();

        while(!self.at_end()) {
            self.start = self.current;
            self.scan_token(&mut vec);
        }

        vec
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn nonliteral_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: self.extract_text().to_string(),
            literal: Literal::None,
            line: self.line
        }
    }

    fn extract_text(&self) -> &str {
        println!("extracting start: {}, finish: {}", self.start, self.current);
        self.source.get(self.start..self.current).unwrap()
    }

    fn advance(&mut self) -> char {
        // EWWW
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn peek(&mut self) -> char {
        if self.at_end() { return '\0' }

        self.source.chars().nth(self.current).unwrap()
    }

    fn check(&mut self, expected: char) -> bool {
        if self.at_end() { return false }

        let c = self.source.chars().nth(self.current).unwrap();
        if c != expected { return false }

        self.current += 1;

        true
    }

    fn nonliteral(&self, token_type: TokenType, tokens: &mut Vec<Token>) {
        tokens.push(
            Token {
                token_type,
                lexeme: self.extract_text().to_string(),
                literal: Literal::None,
                line: self.line
            }
        )
    }

    fn string(&mut self) {}

    fn scan_token(&mut self, tokens: &mut Vec<Token>) {
        let c = self.advance();
        match c {
            '(' => self.nonliteral(TokenType::LeftParen, tokens),
            ')' => self.nonliteral(TokenType::RightParen, tokens),
            '{' => self.nonliteral(TokenType::LeftBrace, tokens),
            '}' => self.nonliteral(TokenType::RightBrace, tokens),
            ',' => self.nonliteral(TokenType::Comma, tokens),
            '.' => self.nonliteral(TokenType::Dot, tokens),
            '-' => self.nonliteral(TokenType::Minus, tokens),
            '+' => self.nonliteral(TokenType::Plus, tokens),
            ';' => self.nonliteral(TokenType::Semicolon, tokens),
            '*' => self.nonliteral(TokenType::Star, tokens),
            '!' => {
                let token = if self.check('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.nonliteral(token, tokens)
            },
            '=' => {
                let token = if self.check('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.nonliteral(token, tokens)
            },
            '<' => {
                let token = if self.check('=') { TokenType::LessEqual } else { TokenType::Less };
                self.nonliteral(token, tokens)
            },
            '>' => {
                let token = if self.check('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.nonliteral(token, tokens)
            },
            '/' => {
                if self.check('/') {
                    // A comment goes until the end of the line.
                    while (self.peek() != '\n' && !self.at_end()) { self.advance(); }
                } else {
                    tokens.push(self.nonliteral_token(TokenType::Slash))
                }
            },
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            },
            '"' => self.string(),
            _ => { self.errors.push(ExecutionError { line: self.line, location: "".to_string(), message: format!("Unexpected character: {}", c) }) }
        }
    }
}
