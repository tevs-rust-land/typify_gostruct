use crate::scanner::{Token, TokenWithContext};
use crate::treewalk::new_ast::{ParseError, RequiredElements, StructDeclaration, AST};
use std::iter::Peekable;

use super::new_ast;

macro_rules! consume_expected_token_with_action {
    ($tokens:expr, $expected:pat, $transform_token:expr, $required_element:expr) => {
        match $tokens.peek().map(|t| &t.token) {
            Some($expected) => {
                let _ = $tokens.next();
                Ok($transform_token)
            }
            Some(_) => {
                let token = $tokens.next().unwrap();
                Err(ParseError::Missing(
                    $required_element,
                    token.lexeme.clone(),
                    token.position,
                ))
            }
            None => Err(ParseError::UnexpectedEndOfFile),
        }
    };
}

macro_rules! consume_expected_token {
    ($tokens:expr, $expected:pat, $required_element:expr) => {
        consume_expected_token_with_action!($tokens, $expected, (), $required_element)
    };
}

fn consume_expected_identifier<'a, I>(tokens: &mut Peekable<I>) -> Result<String, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    consume_expected_token_with_action!(
        tokens,
        &Token::Identifier(ref identifier),
        identifier.to_string(),
        RequiredElements::Identifier
    )
}

pub fn parse(tokens: &[TokenWithContext]) -> Result<Vec<AST>, Vec<String>> {
    let mut statements = Vec::new();
    // let mut errors = Vec::new();
    let mut peekable_tokens = tokens.iter().peekable();
    loop {
        let result = parse_declaration(&mut peekable_tokens);
    }
    Ok(statements)
}

fn parse_declaration<'a, I>(tokens: &mut Peekable<I>) -> Result<AST, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let element = tokens
        .peek()
        .ok_or_else(|| Err(ParseError::UnexpectedEndOfFile))?;
    match element.token {
        Token::Type => {
            let _ = tokens.next();
            parse_struct_declaration(tokens)
        }
        Token::Identifier(key) => {
            let _ = tokens.next();
            parse_identifier(key.to_string(), tokens)
        }
        Token::LeftBrace => {
            let _ = tokens.next();
            parse_block(tokens)
        }
        Token::Json => {
            let _ = tokens.next();
            parse_json(tokens)
        }
        Token::Binding => {
            let _ = tokens.next();
            parse_binding(tokens)
        }
        _ => {
            let parse_error = ParseError::UnknownElement(element.lexeme);
            let error = new_ast::Error::ParseError(parse_error);
            let _ = tokens.next();
            Ok(AST::Error(error))
        }
    }
}

fn parse_struct_declaration<'a, I>(tokens: &mut Peekable<I>) -> Result<AST, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let identifier = consume_expected_identifier(tokens)?;
    consume_expected_token!(tokens, &Token::Struct, RequiredElements::Struct)?;
    consume_expected_token!(tokens, &Token::LeftBrace, RequiredElements::LeftBrace)?;
    let block = parse_block(tokens)?;
    let declaration = StructDeclaration {
        name: identifier,
        body: block,
    };
    Ok(AST::Declaration(Box::new(declaration)))
}

fn parse_block<'a, I>(tokens: &mut Peekable<I>) -> Result<Vec<AST>, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let mut statements = Vec::new();
    fn is_block_end(t: Option<&&TokenWithContext>) -> bool {
        matches!(
            t,
            Some(&TokenWithContext {
                token: Token::RightBrace,
                ..
            })
        )
    };
    while !is_block_end(tokens.peek()) {
        let statement = parse_declaration(tokens)?;
        statements.push(statement)
    }
    let _ = tokens.next();
    Ok(statements)
}
