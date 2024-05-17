use crate::{state::State, tape::Tape, transition::Transition};
use std::collections::HashMap;

pub struct TuringMach {
    tape: Tape,
    current_state: State,
    transition: HashMap<(State, char), Transition>,
}
impl TuringMach {
    pub fn new(tape: Tape, transition: Vec<Transition>) -> Self {
        let mut transition_map = HashMap::new();
        for t in transition {
            transition_map.insert((t.current_state, t.current_char), t);
        }
        Self {
            tape,
            current_state: State::Start,
            transition: transition_map,
        }
    }
    pub fn step(&mut self) -> bool {
        let read_symbol = self.tape.read();
        if let Some(transit) = self.transition.get(&(self.current_state, read_symbol)) {
            self.tape.write(transit.new_char);
            self.tape.move_head_ptr(transit.direction);
            self.current_state = transit.new_state;
            true
        } else {
            false
        }
    }
    pub fn run(&mut self) {
        while self.step() {
            if self.current_state == State::Accept || self.current_state == State::Reject {
                break;
            }
        }
    }
    pub fn is_accepted(&self) -> bool {
        self.current_state == State::Accept
    }
}
