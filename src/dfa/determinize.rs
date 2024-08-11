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

        let is_match = start_state_ids.iter().any(|&id| self.nfa.is_accept(id));
        let start_id = self.dfa.new_state(is_match, &start_state_ids)?;

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
                    let is_match = closure.iter().any(|&id| self.nfa.is_accept(id));
                    let new_id = self.dfa.new_state(is_match, &closure)?;

                    dfa_states.insert(closure_set.clone(), new_id);
                    queue.push_back(new_id);
                    new_id
                };

                self.dfa
                    .add_transition(current_state_id, input, to_state_id);
            }
        }

        Ok(())
    }

    fn get_transitions(&self, state_id: StateID) -> HashMap<char, BTreeSet<nfa::StateID>> {
        let mut transitions: HashMap<char, BTreeSet<nfa::StateID>> = HashMap::new();
        let state = self.dfa.state(state_id).unwrap();

        for &nfa_state_id in &state.nfa_states {
            if let Some(transitions_from_state) =
                self.nfa.state(nfa_state_id).unwrap().as_transitions()
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

                if let Some(transitions) = self.nfa.state(state_id).unwrap().as_transitions() {
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::super::State;
    use super::*;
    use crate::nfa::TransitionKind;
    #[test]
    fn test_build() {
        let mut nfa = NFA::new();
        let _start = nfa.add_state(nfa::State::Transition(vec![
            nfa::Transition {
                to_id: 1,
                kind: TransitionKind::Literal('0'),
            },
            nfa::Transition {
                to_id: 2,
                kind: TransitionKind::Epsilon,
            },
        ]));
        let _one = nfa.add_state(nfa::State::Transition(vec![
            nfa::Transition {
                to_id: 1,
                kind: TransitionKind::Literal('1'),
            },
            nfa::Transition {
                to_id: 3,
                kind: TransitionKind::Literal('1'),
            },
        ]));
        let _two = nfa.add_state(nfa::State::Transition(vec![
            nfa::Transition {
                to_id: 1,
                kind: TransitionKind::Epsilon,
            },
            nfa::Transition {
                to_id: 3,
                kind: TransitionKind::Literal('0'),
            },
        ]));

        let _three = nfa.add_state(nfa::State::Transition(vec![nfa::Transition {
            to_id: 2,
            kind: TransitionKind::Literal('0'),
        }]));

        nfa.make_accept(2).unwrap();
        nfa.make_accept(3).unwrap();

        nfa.print();

        let mut determinizer = Determinizer::new(&nfa);
        determinizer.build().unwrap();

        // 構築されたDFAが期待通りであるかを確認
        let dfa = &determinizer.dfa;

        // 期待される状態と遷移を定義
        let expected_states = vec![
            State {
                id: 0,
                is_match: true,
                nfa_states: vec![0, 2, 1],
            },
            State {
                id: 1,
                is_match: true,
                nfa_states: vec![3, 1],
            },
            State {
                id: 2,
                is_match: true,
                nfa_states: vec![2, 1],
            },
            State {
                id: 3,
                is_match: true,
                nfa_states: vec![3],
            },
        ];

        let expected_transitions = vec![
            (0, '1', 1),
            (0, '0', 1),
            (1, '1', 1),
            (1, '0', 2),
            (2, '0', 3),
            (2, '1', 1),
            (3, '0', 2),
        ];

        // 状態が一致するかを確認
        for expected_state in expected_states.clone() {
            let state = dfa.state(expected_state.id).unwrap();
            assert_eq!(state.is_match, expected_state.is_match);
            assert_eq!(state.nfa_states, expected_state.nfa_states);
        }

        // 遷移が一致するかを確認
        for (from, input, to) in expected_transitions {
            assert_eq!(dfa.next(from, input), Some(to));
        }

        // 受理状態が一致するかを確認
        let expected_accepts: Vec<StateID> = expected_states.iter().map(|s| s.id).collect();
        assert_eq!(dfa.accepts(), expected_accepts);
    }
}
