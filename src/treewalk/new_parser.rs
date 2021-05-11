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
    let mut errors = Vec::new();
    let mut peekable_tokens = tokens.iter().peekable();
    loop {
        let result = parse_declaration(&mut peekable_tokens);
        match result {
            Ok(statement) => statements.push(statement),
            Err(ParseError::UnexpectedEndOfFile) => {
                break;
            }
            Err(error) => {
                errors.push(format!("{}", error));
            }
        }
    }
    if errors.is_empty() {
        Ok(statements)
    } else {
        Err(errors)
    }
}

fn parse_declaration<'a, I>(tokens: &mut Peekable<I>) -> Result<AST, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let element = tokens.peek().ok_or(ParseError::UnexpectedEndOfFile)?;
    match &element.token {
        Token::Type => {
            let _ = tokens.next();
            parse_struct_declaration(tokens)
        }
        Token::Identifier(key) => {
            let _ = tokens.next();
            parse_identifier(key.to_string(), tokens)
        }
        _ => {
            println!("Here");
            let parse_error = ParseError::UnknownElement(element.lexeme.clone());
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

fn parse_identifier<'a, I>(identifier: String, tokens: &mut Peekable<I>) -> Result<AST, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let item = tokens.peek().ok_or(ParseError::UnexpectedEndOfFile)?;
    let item = match &item.token {
        Token::DataType(specified_type) => {
            let _ = tokens.next();
            let field_name = new_ast::FieldName(identifier);
            let field_type = new_ast::FieldType::One(specified_type.clone());
            let field = new_ast::Field::Plain(field_name, field_type);
            Ok(AST::Field(field))
        }
        Token::Identifier(literal) => {
            let _ = tokens.next();

            let field_name = new_ast::FieldName(identifier);
            let field_type =
                new_ast::FieldType::One(new_ast::DataType::Custom(literal.to_string()));
            let field = new_ast::Field::Plain(field_name, field_type);
            Ok(AST::Field(field))
        }
        Token::NextLine => {
            let _ = tokens.next();
            let field_name = new_ast::FieldName(identifier);
            let field_type = new_ast::FieldType::One(new_ast::DataType::Embedded);
            let field = new_ast::Field::Plain(field_name, field_type);
            Ok(AST::Field(field))
        }
        Token::Graveaccent => {
            let vec = Vec::new();
            let field_name = new_ast::FieldName(identifier);
            // TODO: Check if this type is valid
            let field_type = new_ast::FieldType::One(new_ast::DataType::NotSpecified);
            let field = new_ast::Field::WithWithTags(field_name, field_type, vec);
            Ok(AST::Field(field))
        }
        Token::LeftBracket => {
            let _ = tokens.next();
            let field_name = new_ast::FieldName(identifier);
            // TODO: Check if this type is valid
            let field_type = new_ast::FieldType::List(new_ast::DataType::NotSpecified);
            let field = new_ast::Field::Plain(field_name, field_type);
            Ok(AST::Field(field))
        }
        _token => Err(ParseError::UnknownElement(item.lexeme.to_string())),
    };

    let item = item?;

    parse_identifier_to_backticks(item, tokens)
}

fn parse_identifier_to_backticks<'a, I>(
    prev_item: AST,
    tokens: &mut Peekable<I>,
) -> Result<AST, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let current_element = tokens.peek().ok_or(ParseError::UnexpectedEndOfFile)?;
    let item = match (current_element.token.clone(), prev_item) {
        (
            Token::Graveaccent,
            AST::Field(new_ast::Field::WithWithTags(field_name, field_type, ve)),
        ) => {
            let _ = tokens.next();
            let res = parse_backtick_block(tokens)?;
            Ok(AST::Field(new_ast::Field::WithWithTags(
                field_name, field_type, res,
            )))
        }
        (Token::Graveaccent, AST::Field(new_ast::Field::Plain(field_name, field_type))) => {
            let _ = tokens.next();
            let res = parse_backtick_block(tokens)?;
            Ok(AST::Field(new_ast::Field::WithWithTags(
                field_name, field_type, res,
            )))
        }
        (Token::RightBracket, AST::Field(new_ast::Field::Plain(field_name, _field_type))) => {
            let token = tokens.next().ok_or(ParseError::UnexpectedEndOfFile)?;
            let item = match &token.token {
                Token::DataType(specified_type) => {
                    let _ = tokens.next();
                    let specified_type = new_ast::FieldType::List(specified_type.clone());
                    Ok(AST::Field(new_ast::Field::Plain(
                        field_name,
                        specified_type,
                    )))
                }
                Token::Identifier(custom_type) => {
                    let _ = tokens.next();
                    let specified_type =
                        new_ast::FieldType::One(new_ast::DataType::Custom(custom_type.clone()));
                    Ok(AST::Field(new_ast::Field::Plain(
                        field_name,
                        specified_type,
                    )))
                }
                _ => Err(ParseError::UnexpectedElement(token.lexeme.clone())),
            };
            let item = item?;
            let current_element = tokens.peek().ok_or(ParseError::UnexpectedEndOfFile)?;
            match (item, &current_element.token) {
                (
                    new_ast::AST::Field(new_ast::Field::Plain(field_name, specified_type)),
                    Token::Graveaccent,
                ) => {
                    let _ = tokens.next();
                    let res = parse_backtick_block(tokens)?;
                    let field = new_ast::Field::WithWithTags(field_name, specified_type, res);
                    Ok(new_ast::AST::Field(field))
                }
                (p, Token::NextLine) => {
                    let _ = tokens.next();
                    Ok(p)
                }
                _ => Err(ParseError::UnexpectedElement("Unexpected".to_string())),
            }
        }
        (_, p) => {
            let _ = tokens.next();
            Ok(p)
        }
    };
    item
}

fn parse_backtick_block<'a, I>(tokens: &mut Peekable<I>) -> Result<Vec<AST>, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let statements = Vec::new();
    fn is_block_end(t: Option<&&TokenWithContext>) -> bool {
        matches!(
            t,
            Some(&TokenWithContext {
                token: Token::Graveaccent,
                ..
            })
        )
    };
    while !is_block_end(tokens.peek()) {

        // let statement = parse_declaration(tokens)?;
        // statements.push(statement)
    }
    if is_block_end(tokens.peek()) {
        let _ = tokens.next();
        Ok(statements)
    } else {
        Err(ParseError::UnexpectedEndOfFile)
    }
}
