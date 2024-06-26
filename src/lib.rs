mod ast;
mod dfa;
mod error;
mod nfa;

use crate::error::Error;

pub struct Regex {
    dfa: dfa::DFA<usize>,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, Error> {
        let mut parser = ast::parser::Parser::new(pattern);
        let ast = parser.parse()?;

        let mut nfa = nfa::NFA::new();
        nfa.build(&ast)?;

        let mut determinizer = dfa::determinize::Determinizer::new(&nfa);
        determinizer.build()?;

        Ok(Regex {
            dfa: determinizer.dfa,
        })
    }

    pub fn is_match(&self, text: &str) -> bool {
        let mut current_state = self.dfa.start().unwrap();
        for c in text.chars() {
            if let Some(state) = self.dfa.next(current_state, c) {
                current_state = state;
            } else {
                return false;
            }
        }
        self.dfa.accepts().contains(&current_state)
    }
}
