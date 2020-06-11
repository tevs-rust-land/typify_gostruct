use std::iter::Peekable;
use std::rc::Rc;

use crate::scanner::scanner::*;
use crate::treewalk::ast::*;

macro_rules! try_wrap_err {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        }
    };
}

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

#[derive(Debug)]
pub enum RequiredElement {
    Identifier,
    Block,
    Struct,
    Colon,
    StringLiteral,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEndOfFile,
    UnknownError,
    Missing(RequiredElement, Lexeme, Position),
}

pub fn parse(tokens: &[TokenWithContext]) -> Result<Vec<GoStruct>, Vec<ParseError>> {
    let mut statements = Vec::new();
    let mut errors = Vec::new();
    let mut peekable_tokens = tokens.iter().peekable();
    while let Some(result) = parse_declaration(&mut peekable_tokens) {
        match result {
            Ok(statement) => {
                statements.push(statement);
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }
    if errors.is_empty() {
        Ok(statements)
    } else {
        Err(errors)
    }
}

fn parse_declaration<'a, I>(tokens: &mut Peekable<I>) -> Option<Result<GoStruct, ParseError>>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    match tokens.peek().map(|t| &t.token) {
        Some(&Token::Type) => {
            let _ = tokens.next();
            parse_struct_declaration(tokens)
        }
        Some(Token::Identifier(key)) => {
            let _ = tokens.next();
            parse_identifier(key.to_string(), tokens)
        }
        Some(&Token::LeftBrace) => {
            let _ = tokens.next();
            parse_block(tokens)
        }
        Some(&Token::Json) => {
            let _ = tokens.next();
            parse_json(tokens)
        }
        Some(&Token::Binding) => {
            let _ = tokens.next();
            parse_binding(tokens)
        }
        Some(_) => {
            let _ = tokens.next();
            Some(Ok(GoStruct::Unknown))
        }
        None => None,
    }
}

fn parse_struct_declaration<'a, I>(tokens: &mut Peekable<I>) -> Option<Result<GoStruct, ParseError>>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let identifier = try_wrap_err!(consume_expected_identifier(tokens));
    try_wrap_err!(consume_expected_token!(
        tokens,
        &Token::Struct,
        RequiredElement::Struct
    ));
    try_wrap_err!(consume_expected_token!(
        tokens,
        &Token::LeftBrace,
        RequiredElement::Block
    ));
    let block = match parse_block(tokens) {
        Some(Ok(block)) => block,
        Some(err) => return Some(err),
        None => return Some(Err(ParseError::UnexpectedEndOfFile)),
    };
    Some(Ok(GoStruct::StructDefinition(Rc::new(StructDefinition {
        name: identifier,
        body: block,
    }))))
}

fn consume_expected_identifier<'a, I>(tokens: &mut Peekable<I>) -> Result<String, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    consume_expected_token_with_action!(
        tokens,
        &Token::Identifier(ref identifier),
        identifier.to_string(),
        RequiredElement::Identifier
    )
}

fn parse_block<'a, I>(tokens: &mut Peekable<I>) -> Option<Result<GoStruct, ParseError>>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let mut statements = Vec::new();
    fn is_block_end(t: Option<&&TokenWithContext>) -> bool {
        if let Some(&TokenWithContext {
            token: Token::RightBrace,
            ..
        }) = t
        {
            return true;
        } else {
            return false;
        }
    };
    while !is_block_end(tokens.peek()) {
        match parse_declaration(tokens) {
            Some(Ok(statement)) => statements.push(statement),
            None => return Some(Err(ParseError::UnknownError)),
            Some(Err(error)) => return Some(Err(error)),
        }
    }
    if is_block_end(tokens.peek()) {
        let _ = tokens.next();
        Some(Ok(GoStruct::Block(Box::new(Block { statements }))))
    } else {
        Some(Err(ParseError::UnexpectedEndOfFile))
    }
}

fn parse_identifier<'a, I>(
    identifier: String,
    tokens: &mut Peekable<I>,
) -> Option<Result<GoStruct, ParseError>>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let item = match tokens.peek().map(|t| &t.token) {
        Some(&Token::DataType(typ)) => {
            let _ = tokens.next();
            Some(Ok(GoStruct::StructNameWithTypeOnly(identifier, typ)))
        }
        Some(Token::Identifier(literal)) => {
            let _ = tokens.next();
            Some(Ok(GoStruct::StructWithIdentifierTypeOnly(
                identifier,
                literal.to_string(),
            )))
        }
        Some(&Token::NextLine) => {
            let _ = tokens.next();
            Some(Ok(GoStruct::StructNameOnly(identifier)))
        }
        Some(&Token::Graveaccent) => {
            let vec = Vec::new();
            Some(Ok(GoStruct::StructWithJSONTags(
                identifier,
                DataTypeEnum::TypeAny,
                vec,
            )))
        }
        Some(&Token::LeftBracket) => {
            let _ = tokens.next();
            Some(Ok(GoStruct::StructWithList(identifier)))
        }
        Some(_) => Some(Err(ParseError::UnknownError)),
        None => Some(Err(ParseError::UnexpectedEndOfFile)),
    };

    parse_identifier_to_backticks(item, tokens)
}

fn parse_identifier_to_backticks<'a, I>(
    prev_item: Option<Result<GoStruct, ParseError>>,
    tokens: &mut Peekable<I>,
) -> Option<Result<GoStruct, ParseError>>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let item = match (tokens.peek().map(|t| &t.token), prev_item) {
        (Some(&Token::Graveaccent), Some(Ok(GoStruct::StructWithJSONTags(name, typ, _)))) => {
            let _ = tokens.next();
            let res = parse_backtick_block(tokens);
            match res {
                Some(Ok(GoStruct::Block(b))) => {
                    Some(Ok(GoStruct::StructWithJSONTags(name, typ, b.statements)))
                }
                _ => Some(Err(ParseError::UnexpectedEndOfFile)),
            }
        }
        (Some(&Token::Graveaccent), Some(Ok(GoStruct::StructNameWithTypeOnly(name, typ)))) => {
            let _ = tokens.next();
            let res = parse_backtick_block(tokens);
            match res {
                Some(Ok(GoStruct::Block(b))) => {
                    Some(Ok(GoStruct::StructWithJSONTags(name, typ, b.statements)))
                }
                _ => Some(Err(ParseError::UnexpectedEndOfFile)),
            }
        }
        (
            Some(&Token::Graveaccent),
            Some(Ok(GoStruct::StructWithIdentifierTypeOnly(name, literal))),
        ) => {
            let _ = tokens.next();
            let res = parse_backtick_block(tokens);
            match res {
                Some(Ok(GoStruct::Block(b))) => Some(Ok(
                    GoStruct::StructWithIdentifierAndJSONTags(name, literal, b.statements),
                )),
                _ => Some(Err(ParseError::UnexpectedEndOfFile)),
            }
        }
        (Some(&Token::RightBracket), Some(Ok(GoStruct::StructWithList(identifier)))) => {
            let _ = tokens.next();
            let item_type = match tokens.peek().map(|t| &t.token) {
                Some(&Token::DataType(typ)) => {
                    let _ = tokens.next();
                    Some(Ok(GoStruct::StructWithListAndType(identifier, typ)))
                }
                Some(Token::Identifier(customtype)) => {
                    let _ = tokens.next();
                    Some(Ok(GoStruct::StructWithCustomListIdentifier(
                        identifier,
                        customtype.to_string(),
                    )))
                }
                _ => {
                    let _ = tokens.next();
                    Some(Err(ParseError::UnexpectedEndOfFile))
                }
            };
            match (item_type, tokens.peek().map(|t| &t.token)) {
                (
                    Some(Ok(GoStruct::StructWithListAndType(identifier, typ))),
                    Some(&Token::Graveaccent),
                ) => {
                    let _ = tokens.next();
                    let res = parse_backtick_block(tokens);
                    match res {
                        Some(Ok(GoStruct::Block(b))) => Some(Ok(
                            GoStruct::StructWithListTypeAndJSONTags(identifier, typ, b.statements),
                        )),
                        _ => Some(Err(ParseError::UnexpectedEndOfFile)),
                    }
                }
                (
                    Some(Ok(GoStruct::StructWithCustomListIdentifier(identifier, customtype))),
                    Some(&Token::Graveaccent),
                ) => {
                    let _ = tokens.next();
                    let res = parse_backtick_block(tokens);
                    match res {
                        Some(Ok(GoStruct::Block(b))) => {
                            Some(Ok(GoStruct::StructWithCustomListIdentifierAndJSONTags(
                                identifier,
                                customtype,
                                b.statements,
                            )))
                        }
                        _ => Some(Err(ParseError::UnexpectedEndOfFile)),
                    }
                }
                (p, Some(&Token::NextLine)) => {
                    let _ = tokens.peek();
                    p
                }
                _ => {
                    let _ = tokens.peek();
                    Some(Err(ParseError::UnexpectedEndOfFile))
                }
            }
        }
        (_, p) => p,
    };
    item
}

fn parse_backtick_block<'a, I>(tokens: &mut Peekable<I>) -> Option<Result<GoStruct, ParseError>>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let mut statements = Vec::new();
    fn is_block_end(t: Option<&&TokenWithContext>) -> bool {
        if let Some(&TokenWithContext {
            token: Token::Graveaccent,
            ..
        }) = t
        {
            return true;
        } else {
            return false;
        }
    };
    while !is_block_end(tokens.peek()) {
        match parse_declaration(tokens) {
            Some(Ok(statement)) => statements.push(statement),
            None => return Some(Err(ParseError::UnknownError)),
            Some(Err(error)) => return Some(Err(error)),
        }
    }
    if is_block_end(tokens.peek()) {
        let _ = tokens.next();
        Some(Ok(GoStruct::Block(Box::new(Block { statements }))))
    } else {
        Some(Err(ParseError::UnexpectedEndOfFile))
    }
}

fn parse_json<'a, I>(tokens: &mut Peekable<I>) -> Option<Result<GoStruct, ParseError>>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    try_wrap_err!(consume_expected_token!(
        tokens,
        &Token::Colon,
        RequiredElement::Colon
    ));

    let str_literal = try_wrap_err!(consume_expected_token_with_action!(
        tokens,
        &Token::StringLiteral(ref literal),
        literal.to_string(),
        RequiredElement::StringLiteral
    ));
    Some(Ok(GoStruct::JSONName(str_literal)))
}

fn parse_binding<'a, I>(tokens: &mut Peekable<I>) -> Option<Result<GoStruct, ParseError>>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    try_wrap_err!(consume_expected_token!(
        tokens,
        &Token::Colon,
        RequiredElement::Colon
    ));

    try_wrap_err!(consume_expected_token_with_action!(
        tokens,
        &Token::StringLiteral(ref literal),
        literal.to_string(),
        RequiredElement::StringLiteral
    ));
    Some(Ok(GoStruct::Binding))
}
