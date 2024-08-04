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

#[derive(Debug, PartialEq)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        if self.position >= self.input.len() {
            return Token::EndOfFile;
        }

        let current_char = self.input[self.position..].chars().next().unwrap();
        self.position += current_char.len_utf8();

        match current_char {
            '|' => Token::AlternateOperator,
            '*' => Token::StarOperator,
            '+' => Token::PlusOperator,
            '?' => Token::QuestionOperator,
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            _ => Token::Character(current_char),
        }
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
