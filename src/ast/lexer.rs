use std::str::Chars;

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.chars.next().map_or(Token::EndOfFile, Token::from)
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
}
