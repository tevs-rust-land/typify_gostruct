use crate::ast::{Field, ParseError, RequiredElements, StructDeclaration, TagKey, TagValue, AST};
use crate::scanner::{Token, TokenWithContext};
use std::collections::HashMap;
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

fn consume_expected_string_literal<'a, I>(tokens: &mut Peekable<I>) -> Result<String, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    consume_expected_token_with_action!(
        tokens,
        &Token::StringLiteral(ref literal),
        literal.to_string(),
        RequiredElements::StringLiteral
    )
}

pub fn parse(tokens: &[TokenWithContext]) -> Result<Vec<AST>, Vec<String>> {
    let mut statements = Vec::new();
    let mut errors = Vec::new();
    let mut peekable_tokens = tokens.iter().peekable();
    loop {
        let result = parsing_entrypoint(&mut peekable_tokens);
        match result {
            Ok(statement) => statements.push(statement),
            Err(ParseError::UnexpectedEndOfFile) => {
                break;
            }
            Err(error) => {
                errors.push(format!("{}", error));
                // TODO: Enable synchronization for this errors.
                break;
            }
        }
    }
    if errors.is_empty() {
        Ok(statements)
    } else {
        Err(errors)
    }
}

fn parsing_entrypoint<'a, I>(tokens: &mut Peekable<I>) -> Result<AST, ParseError>
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
            parsing_entrypoint(tokens)
        }
        _ => Err(ParseError::UnknownElement(element.lexeme.clone())),
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

fn parse_struct_body<'a, I>(tokens: &mut Peekable<I>) -> Result<Vec<Field>, ParseError>
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
        let statement = parse_struct_field(tokens)?;
        statements.push(statement)
    }

    let _ = tokens.next();
    Ok(statements)
}

fn parse_struct_field<'a, I>(tokens: &mut Peekable<I>) -> Result<Field, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let element = tokens.peek().ok_or(ParseError::UnexpectedEndOfStruct)?;

    match &element.token {
        Token::Identifier(identifier) => {
            let _ = tokens.next();
            let (field_type, field_tags) = parse_field_type_with_tags(tokens)?;
            let field_name = ast::FieldName(identifier.to_string());

            if field_tags.is_empty() {
                let field = ast::Field::Plain(field_name, field_type);
                Ok(field)
            } else {
                Ok(ast::Field::WithTags(field_name, field_type, field_tags))
            }
        }
        Token::NextLine => {
            let _ = tokens.next();
            parse_struct_field(tokens)
        }
        Token::RightBrace => Ok(ast::Field::Blank),
        _ => Err(ParseError::UnknownElement(element.lexeme.clone())),
    }
}

fn parse_field_type_with_tags<'a, I>(
    tokens: &mut Peekable<I>,
) -> Result<(FieldType, HashMap<TagKey, TagValue>), ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let item = tokens.peek().ok_or(ParseError::UnexpectedEndOfFile)?;
    match &item.token {
        Token::DataType(specified_type) => {
            let _ = tokens.next();
            let field_type = ast::FieldType::One(specified_type.clone());
            let field_tags = parse_field_tags_if_present(tokens)?;
            Ok((field_type, field_tags))
        }
        Token::Identifier(literal) => {
            let _ = tokens.next();

            let field_type = ast::FieldType::One(ast::DataType::Custom(literal.to_string()));
            let field_tags = parse_field_tags_if_present(tokens)?;

            Ok((field_type, field_tags))
        }
        Token::NextLine => {
            let _ = tokens.next();
            let field_type = ast::FieldType::One(ast::DataType::Embedded);
            Ok((field_type, HashMap::new()))
        }
        Token::Graveaccent => {
            let _ = tokens.next();
            let field_type = ast::FieldType::One(ast::DataType::Embedded);
            let res = parse_field_tags(tokens)?;
            Ok((field_type, res))
        }
        Token::LeftBracket => {
            let _ = tokens.next();
            consume_expected_token!(tokens, &Token::RightBracket, RequiredElements::RightBracket)?;
            let field_type = parse_type_of_list_with_field(tokens)?;
            let field_tags = parse_field_tags_if_present(tokens)?;
            Ok((field_type, field_tags))
        }
        _token => Err(ParseError::UnknownElement(item.lexeme.to_string())),
    }
}

fn parse_field_tags<'a, I>(
    tokens: &mut Peekable<I>,
) -> Result<HashMap<TagKey, TagValue>, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let mut json_tags = HashMap::new();

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
        let identifier = consume_expected_identifier(tokens)?;
        let identifier = TagKey(identifier);
        consume_expected_token!(tokens, &Token::Colon, RequiredElements::Colon)?;
        let tag_value = consume_expected_string_literal(tokens)?;
        let tag_value = TagValue(tag_value);
        json_tags.insert(identifier, tag_value);
    }
    let _ = tokens.next();
    Ok(json_tags)
}

fn parse_type_of_list_with_field<'a, I>(tokens: &mut Peekable<I>) -> Result<FieldType, ParseError>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let token = tokens.peek().ok_or(ParseError::UnexpectedEndOfFile)?;
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

fn parse_field_tags_if_present<'a, I>(
    tokens: &mut Peekable<I>,
) -> Result<HashMap<TagKey, TagValue>, ParseError>
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
            Ok(HashMap::new())
        }
        _ => Err(ParseError::UnexpectedElement(
            current_element.lexeme.clone(),
        )),
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        parser::parse,
        scanner::{self},
    };

    #[test]
    #[should_panic]
    fn test_with_unclosed_brace() {
        let invalid_example = r#"
        type Region struct {
          Country string `json:"country" binding:"required"`
          State string 
    "#;
        let tokens = scanner::scan(invalid_example).expect("to be scanned correctly");
        parse(&tokens).expect("should panic because of unclosed brace");
    }

    #[test]
    fn test_error_returned_with_unclosed_brace() {
        let invalid_example = r#"
        type Region struct {
          Country string `json:"country" binding:"required"`
          State string 
    "#;
        let tokens = scanner::scan(invalid_example).expect("to be scanned correctly");
        let parsed_result = parse(&tokens);
        match parsed_result {
            Ok(_ast) => panic!("This test should not pass"),
            Err(err) => {
                assert_eq!(err.len(), 1);
            }
        }
    }
    #[test]
    fn test_should_parse_valid_struct_correctly() {
        let valid_struct = r#"
        type Region struct {
          Country string `json:"country" binding:"required"`
          State string 
        }
    "#;
        let tokens = scanner::scan(valid_struct).expect("to be scanned correctly");
        let parsed_result = parse(&tokens).expect("The struct should be parsed correctly");
        assert_eq!(parsed_result.len(), 1)
    }

    #[test]
    fn test_should_parse_struct_with_list_correctly() {
        let valid_struct = r#"
            type List struct {
                People    []*Person
            }
        "#;
        let tokens = scanner::scan(valid_struct).expect("to be scanned correctly");

        let parsed_result = parse(&tokens);
        assert!(parsed_result.is_ok())
    }

    #[test]
    fn test_should_parse_struct_with_list_and_json_tags_correctly() {
        let valid_struct = r#"
            type List struct {
                People    []*Person `json:"people"`
            }
        "#;
        let tokens = scanner::scan(valid_struct).expect("to be scanned correctly");

        let parsed_result = parse(&tokens);
        assert!(parsed_result.is_ok())
    }
}
