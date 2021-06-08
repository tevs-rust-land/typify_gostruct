use crate::ast::{ParseError, RequiredElements, StructDeclaration, AST};
use crate::scanner::{Token, TokenWithContext};
use std::iter::Peekable;

use super::ast::{self, FieldType};

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
        Token::NextLine => {
            let _ = tokens.next();
            parse_declaration(tokens)
        }
        _ => {
            let parse_error = ParseError::UnknownElement(element.lexeme.clone());
            let error = ast::Error::ParseError(parse_error);
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
    let block = parse_struct_body(tokens)?;
    let declaration = StructDeclaration {
        name: identifier,
        body: block,
    };
    Ok(AST::Declaration(Box::new(declaration)))
}

fn parse_struct_body<'a, I>(tokens: &mut Peekable<I>) -> Result<Vec<AST>, ParseError>
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
        let statement = parse_struct_fields(tokens)?;
        statements.push(statement)
    }
    let _ = tokens.next();
    Ok(statements)
}

fn parse_struct_fields<'a, I>(tokens: &mut Peekable<I>) -> Result<AST, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let element = tokens.peek().ok_or(ParseError::UnexpectedEndOfFile)?;
    match &element.token {
        Token::Identifier(identifier) => {
            let _ = tokens.next();
            let (field_type, field_tags) = parse_field_type_with_tags(tokens)?;
            let field_name = ast::FieldName(identifier.to_string());

            if field_tags.is_empty() {
                let field = ast::Field::Plain(field_name, field_type);
                Ok(AST::Field(field))
            } else {
                Ok(AST::Field(ast::Field::WithTags(
                    field_name, field_type, field_tags,
                )))
            }
        }
        Token::NextLine => {
            let _ = tokens.next();
            parse_struct_fields(tokens)
        }
        _ => {
            let parse_error = ParseError::UnknownElement(element.lexeme.clone());
            let error = ast::Error::ParseError(parse_error);
            let _ = tokens.next();
            Ok(AST::Error(error))
        }
    }
}

fn parse_field_type_with_tags<'a, I>(
    tokens: &mut Peekable<I>,
) -> Result<(FieldType, Vec<AST>), ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let item = tokens.peek().ok_or(ParseError::UnexpectedEndOfFile)?;
    match &item.token {
        Token::DataType(specified_type) => {
            let _ = tokens.next();
            let field_type = ast::FieldType::One(specified_type.clone());
            Ok((field_type, Vec::new()))
        }
        Token::Identifier(literal) => {
            let _ = tokens.next();

            let field_type = ast::FieldType::One(ast::DataType::Custom(literal.to_string()));
            Ok((field_type, Vec::new()))
        }
        Token::NextLine => {
            let _ = tokens.next();
            let field_type = ast::FieldType::One(ast::DataType::Embedded);
            Ok((field_type, Vec::new()))
        }
        Token::Graveaccent => {
            let _ = tokens.next();
            let field_type = ast::FieldType::One(ast::DataType::Embedded);
            let res = parse_field_tags(tokens)?;
            Ok((field_type, res))
        }
        Token::LeftBracket => {
            let _ = tokens.next();
            let field_type = parse_type_of_list_with_field(tokens)?;
            let struct_tags = parse_json_tags_on_list_field_type(tokens)?;
            Ok((field_type, struct_tags))
        }

        _token => Err(ParseError::UnknownElement(item.lexeme.to_string())),
    }
}

fn parse_field_tags<'a, I>(tokens: &mut Peekable<I>) -> Result<Vec<AST>, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let mut statements = Vec::new();
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
        // TODO: Finish this implementation.. We should have a method that parses the content of field tags, eg.. parse_tags
        let statement = parse_declaration(tokens)?;
        statements.push(statement)
    }
    if is_block_end(tokens.peek()) {
        let _ = tokens.next();
        Ok(statements)
    } else {
        Err(ParseError::UnexpectedEndOfFile)
    }
}

fn parse_type_of_list_with_field<'a, I>(tokens: &mut Peekable<I>) -> Result<FieldType, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let token = tokens.next().ok_or(ParseError::UnexpectedEndOfFile)?;
    match &token.token {
        Token::DataType(specified_type) => {
            let _ = tokens.next();
            let specified_type = ast::FieldType::List(specified_type.clone());
            Ok(specified_type)
        }
        Token::Identifier(custom_type) => {
            let _ = tokens.next();
            let specified_type = ast::FieldType::List(ast::DataType::Custom(custom_type.clone()));
            Ok(specified_type)
        }
        _ => Err(ParseError::UnexpectedElement(token.lexeme.clone())),
    }
}

fn parse_json_tags_on_list_field_type<'a, I>(
    tokens: &mut Peekable<I>,
) -> Result<Vec<AST>, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let current_element = tokens.peek().ok_or(ParseError::UnexpectedEndOfFile)?;

    match &current_element.token {
        Token::Graveaccent => {
            let _ = tokens.next();
            let res = parse_field_tags(tokens)?;
            Ok(res)
        }
        Token::NextLine => {
            let _ = tokens.next();
            Ok(Vec::new())
        }
        _ => Err(ParseError::UnexpectedElement("Unexpected".to_string())),
    }
}

// TODO: Setup factory functions for the astTypes eg.. newFieldWithType() -> AST