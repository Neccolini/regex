use super::lexer::{Lexer, Token};
use super::{Ast, Repetition};
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(pattern: &'a str) -> Self {
        let mut lexer = Lexer::new(pattern);
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }

    pub fn parse(&mut self) -> Result<Ast, Error> {
        self.parse_alternate()
    }

    fn next(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parse_literal(&mut self) -> Result<Ast, Error> {
        match self.current_token {
            Token::Character(c) => {
                self.next();
                Ok(Ast::Literal(c))
            }
            Token::OpenParenthesis => {
                self.next();
                let ast = self.parse_alternate()?;
                if let Token::CloseParenthesis = self.current_token {
                    self.next();
                    Ok(ast)
                } else {
                    Err(Error::parse("Close parenthesis is missing"))
                }
            }
            _ => Err(Error::parse("Unexpected token")),
        }
    }

    fn parse_concat(&mut self) -> Result<Ast, Error> {
        let mut nodes = vec![self.parse_repetition()?];

        while matches!(
            self.current_token,
            Token::Character(_) | Token::OpenParenthesis
        ) {
            nodes.push(self.parse_repetition()?);
        }

        match nodes.len() {
            0 => Err(Error::parse("Expected at least one node in concat")),
            1 => Ok(nodes.pop().unwrap()),
            _ => Ok(Ast::Concat(nodes)),
        }
    }

    fn parse_alternate(&mut self) -> Result<Ast, Error> {
        let mut nodes = vec![self.parse_concat()?];

        while matches!(self.current_token, Token::AlternateOperator) {
            self.next();
            nodes.push(self.parse_concat()?);
        }

        match nodes.len() {
            0 => Err(Error::parse("Expected at least one node in alternate")),
            1 => Ok(nodes.pop().unwrap()),
            _ => Ok(Ast::Alternate(nodes)),
        }
    }

    fn parse_repetition(&mut self) -> Result<Ast, Error> {
        let ast = self.parse_literal()?;

        match self.current_token {
            Token::StarOperator => {
                self.next();
                Ok(Ast::Repetition(Repetition {
                    ast: Box::new(ast),
                    min: 0,
                    max: None,
                }))
            }
            Token::PlusOperator => {
                self.next();
                Ok(Ast::Repetition(Repetition {
                    ast: Box::new(ast),
                    min: 1,
                    max: None,
                }))
            }
            Token::QuestionOperator => {
                self.next();
                Ok(Ast::Repetition(Repetition {
                    ast: Box::new(ast),
                    min: 0,
                    max: Some(1),
                }))
            }
            _ => Ok(ast),
        }
    }
}

#[allow(dead_code)]
fn print_ast(ast: &Ast, indent: usize) {
    let indent_str = " ".repeat(indent);
    match ast {
        Ast::Literal(c) => {
            println!("{}Literal({})", indent_str, c);
        }
        Ast::Concat(concat) => {
            println!("{}Concat:", indent_str);
            concat.iter().for_each(|ast| print_ast(ast, indent + 2));
        }
        Ast::Alternate(alternate) => {
            println!("{}Alternate:", indent_str);
            alternate.iter().for_each(|ast| print_ast(ast, indent + 2));
        }
        Ast::Repetition(repetition) => {
            let max_str = match repetition.max {
                Some(max) => max.to_string(),
                None => "None".to_string(),
            };
            println!(
                "{}Repetition(min: {}, max: {}):",
                indent_str, repetition.min, max_str
            );
            print_ast(&repetition.ast, indent + 2);
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let mut parser = Parser::new("a|b");
        let ast = parser.parse().unwrap();

        // assertion
        assert_eq!(
            ast,
            Ast::Alternate(vec![Ast::Literal('a'), Ast::Literal('b')])
        );

        parser = Parser::new("a|b|c");
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            Ast::Alternate(vec![
                Ast::Literal('a'),
                Ast::Literal('b'),
                Ast::Literal('c')
            ])
        );

        parser = Parser::new("a(bc|d)");
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            Ast::Concat(vec![
                Ast::Literal('a'),
                Ast::Alternate(vec![
                    Ast::Concat(vec![Ast::Literal('b'), Ast::Literal('c')]),
                    Ast::Literal('d')
                ])
            ])
        );

        parser = Parser::new("((a|b)+)*");
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            Ast::Repetition(Repetition {
                ast: Box::new(Ast::Repetition(Repetition {
                    ast: Box::new(Ast::Alternate(vec![
                        Ast::Literal('a'),
                        Ast::Literal('b')
                    ])),
                    min: 1,
                    max: None
                })),
                min: 0,
                max: None
            })
        );
    }
}
