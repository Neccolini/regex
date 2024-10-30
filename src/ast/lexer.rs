use std::{
    iter::{Map, Peekable},
    str::Chars,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Character(char),   // a, b, c, ...
    AlternateOperator, // |
    StarOperator,      // *
    PlusOperator,      // +
    QuestionOperator,  // ?
    OpenParenthesis,   // (
    CloseParenthesis,  // )
    EndOfFile,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '|' => Token::AlternateOperator,
            '*' => Token::StarOperator,
            '+' => Token::PlusOperator,
            '?' => Token::QuestionOperator,
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            _ => Token::Character(value),
        }
    }
}

impl From<Option<Token>> for Token {
    fn from(value: Option<Token>) -> Self {
        value.unwrap_or(Token::EndOfFile)
    }
}

type TokenPeekable<'a> = Peekable<Map<Chars<'a>, fn(char) -> Token>>;

#[derive(Debug)]
pub struct Lexer<'a> {
    token_iter: TokenPeekable<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            token_iter: input
                .chars()
                .map(Into::into as fn(char) -> Token)
                .peekable(),
        }
    }

    pub fn peek_token(&mut self) -> Token {
        self.token_iter.peek().cloned().into()
    }

    pub fn next_token(&mut self) -> Token {
        self.token_iter.next().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("a|b");
        assert_eq!(lexer.next_token(), Token::Character('a'));
        assert_eq!(lexer.next_token(), Token::AlternateOperator);
        assert_eq!(lexer.next_token(), Token::Character('b'));
        assert_eq!(lexer.next_token(), Token::EndOfFile);

        lexer = Lexer::new("a*");
        assert_eq!(lexer.next_token(), Token::Character('a'));
        assert_eq!(lexer.next_token(), Token::StarOperator);
        assert_eq!(lexer.next_token(), Token::EndOfFile);

        lexer = Lexer::new("a+");
        assert_eq!(lexer.next_token(), Token::Character('a'));
        assert_eq!(lexer.next_token(), Token::PlusOperator);
        assert_eq!(lexer.next_token(), Token::EndOfFile);

        lexer = Lexer::new("a?");
        assert_eq!(lexer.next_token(), Token::Character('a'));
        assert_eq!(lexer.next_token(), Token::QuestionOperator);
        assert_eq!(lexer.next_token(), Token::EndOfFile);

        lexer = Lexer::new("(a)");
        assert_eq!(lexer.next_token(), Token::OpenParenthesis);
        assert_eq!(lexer.next_token(), Token::Character('a'));
        assert_eq!(lexer.next_token(), Token::CloseParenthesis);
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }

    #[test]
    fn test_peek() {
        let mut lexer = Lexer::new("abcde");
        assert_eq!(lexer.peek_token(), Token::Character('a'));
        assert_eq!(lexer.peek_token(), Token::Character('a'));
        assert_eq!(lexer.next_token(), Token::Character('a'));
        assert_eq!(lexer.next_token(), Token::Character('b'));
        assert_eq!(lexer.peek_token(), Token::Character('c'));
        lexer.next_token();
        lexer.next_token();
        assert_eq!(lexer.peek_token(), Token::Character('e'));
        assert_eq!(lexer.next_token(), Token::Character('e'));
        assert_eq!(lexer.peek_token(), Token::EndOfFile);
        assert_eq!(lexer.next_token(), Token::EndOfFile);
        assert_eq!(lexer.peek_token(), Token::EndOfFile);
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }
}
