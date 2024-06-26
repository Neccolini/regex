pub mod determinize;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::error::Error;

pub type StateID = usize;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct State<S> {
    pub id: StateID,
    pub is_match: bool,
    pub nfa_states: Vec<S>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct DFA<S: Eq + Hash + Clone> {
    states: Vec<State<S>>,
    start: Option<StateID>,
    accept_states: HashSet<StateID>,
    transitions: HashMap<StateID, HashMap<char, StateID>>,
}

impl<S: Eq + Hash + Clone> DFA<S> {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            start: None,
            accept_states: HashSet::new(),
            transitions: HashMap::new(),
        }
    }

    pub fn set_start(&mut self, state_id: StateID) {
        self.start = Some(state_id);
    }

    pub fn add_accept_state(&mut self, state_id: StateID) {
        self.accept_states.insert(state_id);
    }

    pub fn add_transition(&mut self, from: StateID, input: char, to: StateID) {
        self.transitions.entry(from).or_default().insert(input, to);
    }

    pub fn new_state(&mut self, is_match: bool, nfa_states: &[S]) -> Result<StateID, Error> {
        let id = self.states.len();

        let state = State {
            id,
            is_match,
            nfa_states: nfa_states.to_owned(),
        };

        self.states.push(state);

        Ok(id)
    }

    pub fn state(&self, id: StateID) -> Option<&State<S>> {
        self.states.get(id)
    }

    pub fn start(&self) -> Option<StateID> {
        self.start
    }
    pub fn accepts(&self) -> &HashSet<StateID> {
        &self.accept_states
    }

    pub fn next(&self, current: StateID, input: char) -> Option<StateID> {
        self.transitions
            .get(&current)
            .and_then(|transitions| transitions.get(&input))
            .cloned()
    }

    pub fn print(&self)
    where
        S: std::fmt::Debug,
    {
        for (i, state) in self.states.iter().enumerate() {
            println!("State {}", i);
            println!("  is_match: {}", state.is_match);
            println!("  nfa_states: {:?}", state.nfa_states);
            println!("  transitions:");
            if let Some(transitions) = self.transitions.get(&state.id) {
                for (input, to) in transitions {
                    println!("    {} -> {}", input, to);
                }
            }
        }
        println!("Accept states: {:?}", self.accept_states);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfa() {
        let mut dfa: DFA<usize> = DFA::new();

        let state0 = dfa.new_state(false, &vec![0]).unwrap();
        let state1 = dfa.new_state(false, &vec![1]).unwrap();
        let state2 = dfa.new_state(true, &vec![2]).unwrap();

        dfa.set_start(state0);
        dfa.add_accept_state(state2);

        dfa.add_transition(state0, 'a', state1);
        dfa.add_transition(state1, 'b', state2);

        assert_eq!(dfa.start(), Some(state0));
        assert_eq!(dfa.accepts(), &vec![state2].into_iter().collect());

        assert_eq!(dfa.next(state0, 'a'), Some(state1));
        assert_eq!(dfa.next(state1, 'b'), Some(state2));
        assert_eq!(dfa.next(state0, 'b'), None);
    }
}
