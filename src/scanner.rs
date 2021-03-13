use itertools::{multipeek, MultiPeek};
use std::str;

use crate::data_types::DataTypeEnum;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftBrace,
    RightBrace,
    Colon,
    Identifier(String),
    StringLiteral(String),
    Whitespace,
    Graveaccent,
    NextLine,
    LeftBracket,
    RightBracket,
    Star,
    // Keywords
    Type,
    Struct,
    Binding,
    Json,
    DataType(DataTypeEnum),
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    column: usize,
}

impl Position {
    fn initial() -> Position {
        Position { line: 1, column: 1 }
    }

    fn increment_column(&mut self) {
        self.column += 1;
    }

    fn increment_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

pub type Lexeme = String;

#[derive(Debug)]
pub struct TokenWithContext {
    pub token: Token,
    pub lexeme: Lexeme,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub enum ScannerError {
    MissingStringTerminator(Position),
    // UnexpectedCharacter(char, Position),
}

struct Scanner<'a> {
    current_position: Position,
    current_lexeme: String,
    source: MultiPeek<str::Chars<'a>>,
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_alpha(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '.'
}

fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

fn is_nextline(c: char) -> bool {
    matches!(c, '\n')
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\r' | '\t')
}

impl<'a> Scanner<'a> {
    fn initialize(source: &'a str) -> Scanner {
        Scanner {
            current_position: Position::initial(),
            current_lexeme: "".into(),
            source: multipeek(source.chars()),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.source.next();
        if let Some(c) = next {
            self.current_lexeme.push(c);
            if c == '\n' {
                self.current_position.increment_line();
            } else {
                self.current_position.increment_column();
            }
        }
        next
    }

    fn peek_check(&mut self, check: &dyn Fn(char) -> bool) -> bool {
        self.source.reset_peek();
        match self.source.peek() {
            Some(&c) => check(c),
            None => false,
        }
    }

    fn advance_if_match(&mut self, expected: char) -> bool {
        if self.peek_check(&|c| c == expected) {
            let _ = self.advance();
            true
        } else {
            false
        }
    }

    fn advance_while(&mut self, condition: &dyn Fn(char) -> bool) {
        while self.peek_check(condition) {
            self.advance();
        }
    }
    fn string(&mut self) -> Result<Token, ScannerError> {
        self.advance_while(&|c| c != '"' && c != '\n');
        if !self.advance_if_match('"') {
            return Err(ScannerError::MissingStringTerminator(self.current_position));
        }
        let literal_length = self.current_lexeme.len() - 2;
        let literal: String = self
            .current_lexeme
            .chars()
            .skip(1)
            .take(literal_length)
            .collect();
        if literal == "binding" {
            Ok(Token::Binding)
        } else {
            Ok(Token::StringLiteral(literal))
        }
    }

    fn identifier(&mut self) -> Token {
        self.advance_while(&is_alphanumeric);
        match self.current_lexeme.as_ref() {
            "type" => Token::Type,
            "struct" => Token::Struct,
            "binding" => Token::Binding,
            "json" => Token::Json,
            // data types
            "int64" => Token::DataType(DataTypeEnum::TypeNumber),
            "float64" => Token::DataType(DataTypeEnum::TypeNumber),
            "string" => Token::DataType(DataTypeEnum::TypeString),
            "null.String" => Token::DataType(DataTypeEnum::TypeNullString),
            "null.Float" => Token::DataType(DataTypeEnum::TypeNullString),
            "null.Int" => Token::DataType(DataTypeEnum::TypeNullNumber),
            "int" => Token::DataType(DataTypeEnum::TypeNumber),
            "time.Time" => Token::DataType(DataTypeEnum::TypeDate),
            "bool" => Token::DataType(DataTypeEnum::TypeBoolean),
            identifier => Token::Identifier(identifier.into()),
        }
    }

    fn add_context(&mut self, token: Token, initial_position: Position) -> TokenWithContext {
        TokenWithContext {
            token,
            lexeme: self.current_lexeme.clone(),
            position: initial_position,
        }
    }

    fn scan_next(&mut self) -> Option<Result<TokenWithContext, ScannerError>> {
        let initial_position = self.current_position;
        self.current_lexeme.clear();

        let next_char = match self.advance() {
            Some(c) => c,
            None => return None,
        };

        let result = match next_char {
            ':' => Ok(Token::Colon),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            '`' => Ok(Token::Graveaccent),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            '*' => Ok(Token::Star),
            c if is_nextline(c) => Ok(Token::NextLine),
            c if is_whitespace(c) => Ok(Token::Whitespace),
            '"' => self.string(),
            _ => Ok(self.identifier()),
        };
        Some(result.map(|token| self.add_context(token, initial_position)))
    }
}

struct TokensIterator<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Iterator for TokensIterator<'a> {
    type Item = Result<TokenWithContext, ScannerError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.scan_next()
    }
}

pub fn scan_into_iterator<'a>(
    source: &'a str,
) -> impl Iterator<Item = Result<TokenWithContext, ScannerError>> + 'a {
    TokensIterator {
        scanner: Scanner::initialize(source),
    }
}

pub fn scan(source: &str) -> (Vec<TokenWithContext>, Vec<ScannerError>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    for result in scan_into_iterator(source) {
        match result {
            Ok(token_with_context) => {
                match token_with_context.token {
                    Token::Whitespace => {}
                    Token::Star => {}
                    _ => tokens.push(token_with_context),
                };
            }
            Err(error) => errors.push(error),
        }
    }
    (tokens, errors)
}
