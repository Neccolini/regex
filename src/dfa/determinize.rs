use std::collections::{BTreeSet, HashMap, VecDeque};

use super::{StateID, DFA};
use crate::error::Error;
use crate::nfa::{self, TransitionKind, NFA};

pub struct Determinizer<'a> {
    nfa: &'a NFA,
    pub dfa: DFA<nfa::StateID>,
}

impl<'a> Determinizer<'a> {
    pub fn new(nfa: &'a NFA) -> Self {
        Self {
            nfa,
            dfa: DFA::new(),
        }
    }

    pub fn build(&mut self) -> Result<(), Error> {
        let mut dfa_states: HashMap<BTreeSet<nfa::StateID>, StateID> = HashMap::new();
        let mut queue: VecDeque<StateID> = VecDeque::new();

        let start_state_ids = self.epsilon_closure(&[self.nfa.start()]);
        let start_state_set: BTreeSet<nfa::StateID> = start_state_ids.iter().cloned().collect();

        let start_id = self.dfa.new_state(false, &start_state_ids)?;
        self.dfa.set_start(start_id);
        dfa_states.insert(start_state_set, start_id);
        queue.push_back(start_id);

        while let Some(current_state_id) = queue.pop_front() {
            let transitions = self.get_transitions(current_state_id);

            for (input, nfa_state_ids) in transitions {
                let closure = self.epsilon_closure(&nfa_state_ids.into_iter().collect::<Vec<_>>());
                let closure_set: BTreeSet<nfa::StateID> = closure.iter().cloned().collect();

                let to_state_id = if let Some(&existing_id) = dfa_states.get(&closure_set) {
                    existing_id
                } else {
                    let new_id = self.dfa.new_state(false, &closure)?;
                    dfa_states.insert(closure_set.clone(), new_id);
                    queue.push_back(new_id);
                    new_id
                };

                self.dfa
                    .add_transition(current_state_id, input, to_state_id);
            }
        }

        for (nfa_states, dfa_state_id) in &dfa_states {
            if nfa_states.contains(&self.nfa.end()) {
                self.dfa.add_accept_state(*dfa_state_id);
            }
        }

        Ok(())
    }

    fn get_transitions(&self, state_id: StateID) -> HashMap<char, BTreeSet<nfa::StateID>> {
        let mut transitions: HashMap<char, BTreeSet<nfa::StateID>> = HashMap::new();
        let state = self.dfa.state(state_id).unwrap();

        for &nfa_state_id in &state.nfa_states {
            if let Some(transitions_from_state) =
                self.nfa.state(nfa_state_id).unwrap().as_transition()
            {
                for transition in transitions_from_state {
                    if let TransitionKind::Literal(c) = transition.kind() {
                        transitions
                            .entry(*c)
                            .or_default()
                            .insert(transition.to_id());
                    }
                }
            }
        }

        transitions
    }

    fn epsilon_closure(&self, start: &[nfa::StateID]) -> Vec<nfa::StateID> {
        let mut closure = Vec::new();
        let mut stack = start.to_vec();
        let mut visited = vec![false; self.nfa.states_count()];

        while let Some(state_id) = stack.pop() {
            if !visited[state_id] {
                closure.push(state_id);
                visited[state_id] = true;

                if let Some(transitions) = self.nfa.state(state_id).unwrap().as_transition() {
                    for transition in transitions {
                        if let nfa::TransitionKind::Epsilon = transition.kind() {
                            stack.push(transition.to_id());
                        }
                    }
                }
            }
        }

        closure
    }
}
