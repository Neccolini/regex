use crate::ast::{Ast, Repetition};
use crate::error::Error;
pub type StateID = usize;

pub enum State {
    Accept(Vec<Transition>),
    Transition(Vec<Transition>),
}

impl State {
    pub fn as_transitions_mut(&mut self) -> &mut Vec<Transition> {
        match self {
            State::Accept(transitions) | State::Transition(transitions) => transitions,
        }
    }

    pub fn as_transitions(&self) -> &Vec<Transition> {
        match self {
            State::Accept(transitions) | State::Transition(transitions) => transitions,
        }
    }

    pub fn make_accept(&mut self) {
        if let State::Transition(transitions) = self {
            *self = State::Accept(transitions.clone());
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Transition {
    pub to_id: StateID,
    pub kind: TransitionKind,
}

impl Transition {
    pub fn to_id(&self) -> StateID {
        self.to_id
    }

    pub fn kind(&self) -> &TransitionKind {
        &self.kind
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TransitionKind {
    Literal(char),
    Epsilon,
}

pub struct NFAFragment {
    start: StateID,
    end: StateID,
}

#[allow(clippy::upper_case_acronyms)]
pub struct NFA {
    start: StateID,
    end: StateID,
    states: Vec<State>,
}

impl NFA {
    pub fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            states: Vec::new(),
        }
    }

    pub fn build(&mut self, ast: &Ast) -> Result<(), Error> {
        let states = self.construct(ast)?;

        // endをAcceptに変更
        self.make_accept(states.end)?;

        self.set_start_end(states.start, states.end);

        Ok(())
    }

    pub fn add_state(&mut self, state: State) -> StateID {
        let id = self.states.len() as StateID;
        self.states.push(state);
        id
    }

    pub fn make_accept(&mut self, id: StateID) -> Result<(), Error> {
        self.states
            .get_mut(id)
            .map(State::make_accept)
            .ok_or(Error::state_id_overflow(self.states_count()))
    }

    pub fn start(&self) -> StateID {
        self.start
    }

    pub fn state(&self, id: StateID) -> Option<&State> {
        self.states.get(id)
    }

    pub fn states_count(&self) -> usize {
        self.states.len()
    }

    pub fn is_accept(&self, id: StateID) -> bool {
        matches!(self.states.get(id), Some(State::Accept(_)))
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("NFA:");
        println!("Start state: {}", self.start);
        println!("End state: {}", self.end);
        for (i, state) in self.states.iter().enumerate() {
            match state {
                State::Transition(transitions) | State::Accept(transitions) => {
                    println!("State {}: Transitions ->", i);
                    for transition in transitions {
                        match transition.kind {
                            TransitionKind::Literal(c) => {
                                println!("  to_id {} on '{}'", transition.to_id, c)
                            }
                            TransitionKind::Epsilon => {
                                println!("  to_id {} on 'ε'", transition.to_id)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl NFA {
    fn add_transition(
        &mut self,
        from_id: StateID,
        to_id: StateID,
        kind: TransitionKind,
    ) -> Result<(), Error> {
        self.states
            .get_mut(from_id)
            .map(|state| state.as_transitions_mut().push(Transition { to_id, kind }))
            .ok_or(Error::state_id_overflow(self.states_count()))
    }

    fn new_fragment(&mut self) -> NFAFragment {
        let start = self.add_state(State::Transition(Vec::new()));
        let end = self.add_state(State::Transition(Vec::new()));
        NFAFragment { start, end }
    }

    fn set_start_end(&mut self, start: StateID, end: StateID) {
        self.start = start;
        self.end = end;
    }

    fn construct(&mut self, ast: &Ast) -> Result<NFAFragment, Error> {
        match ast {
            Ast::Literal(c) => self.construct_literal(*c),
            Ast::Concat(concats) => self.construct_concat(concats),
            Ast::Alternate(alternates) => self.construct_alternate(alternates),
            Ast::Repetition(repetition) => self.construct_repetition(repetition),
        }
    }

    fn construct_literal(&mut self, c: char) -> Result<NFAFragment, Error> {
        let fragment = self.new_fragment();

        self.add_transition(fragment.start, fragment.end, TransitionKind::Literal(c))?;

        Ok(fragment)
    }

    fn construct_concat(&mut self, concats: &[Ast]) -> Result<NFAFragment, Error> {
        let mut concats_iter = concats.iter();

        let mut current_fragment = self.construct(
            concats_iter
                .next()
                .ok_or(Error::syntax("Empty concatenation"))?,
        )?;

        for ast in concats_iter {
            let next_fragment = self.construct(ast)?;

            self.add_transition(
                current_fragment.end,
                next_fragment.start,
                TransitionKind::Epsilon,
            )?;
            current_fragment.end = next_fragment.end;
        }

        Ok(current_fragment)
    }

    fn construct_alternate(&mut self, alternates: &[Ast]) -> Result<NFAFragment, Error> {
        let fragment = self.new_fragment();

        for ast in alternates {
            let alt_fragment = self.construct(ast)?;

            self.add_transition(fragment.start, alt_fragment.start, TransitionKind::Epsilon)?;
            self.add_transition(alt_fragment.end, fragment.end, TransitionKind::Epsilon)?;
        }

        Ok(fragment)
    }

    fn construct_repetition(&mut self, repetition: &Repetition) -> Result<NFAFragment, Error> {
        match (repetition.min, repetition.max) {
            (0, Some(1)) => self.construct_zero_or_one(&repetition.ast),
            (0, None) => self.construct_at_least(&repetition.ast, 0),
            (1, None) => self.construct_at_least(&repetition.ast, 1),
            _ => unimplemented!(),
        }
    }

    fn construct_zero_or_one(&mut self, ast: &Ast) -> Result<NFAFragment, Error> {
        let fragment = self.new_fragment();
        let inner_fragment = self.construct(ast)?;

        self.add_transition(
            fragment.start,
            inner_fragment.start,
            TransitionKind::Epsilon,
        )?;
        self.add_transition(inner_fragment.end, fragment.end, TransitionKind::Epsilon)?;

        // 空文字を受理するε遷移
        self.add_transition(fragment.start, fragment.end, TransitionKind::Epsilon)?;

        Ok(fragment)
    }

    fn construct_at_least(&mut self, ast: &Ast, n: usize) -> Result<NFAFragment, Error> {
        let fragment = self.new_fragment();

        // 繰り返し部分
        let mut inner_fragment = self.construct(ast)?;

        self.add_transition(
            inner_fragment.end,
            inner_fragment.start,
            TransitionKind::Epsilon,
        )?;
        self.add_transition(inner_fragment.end, fragment.end, TransitionKind::Epsilon)?;

        if n == 0 {
            // 空文字受理
            self.add_transition(fragment.start, fragment.end, TransitionKind::Epsilon)?;
        } else {
            for _ in 0..n - 1 {
                let pre_fragment = self.construct(ast)?;

                self.add_transition(
                    pre_fragment.end,
                    inner_fragment.start,
                    TransitionKind::Epsilon,
                )?;

                // inner_fragment を更新
                inner_fragment = pre_fragment;
            }
        }

        // 開始状態から inner_fragment に ε 遷移
        self.add_transition(
            fragment.start,
            inner_fragment.start,
            TransitionKind::Epsilon,
        )?;

        Ok(fragment)
    }
}
