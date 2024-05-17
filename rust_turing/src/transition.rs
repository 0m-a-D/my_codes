use crate::direction::Direction;
use crate::state::State;

pub struct Transition {
    pub current_state: State,
    pub current_char: char,
    pub new_state: State,
    pub new_char: char,
    pub direction: Direction,
}
impl Transition {
    pub fn new(
        current_state: State,
        current_char: char,
        new_state: State,
        new_char: char,
        direction: Direction,
    ) -> Transition {
        Self {
            current_state,
            current_char,
            new_state,
            new_char,
            direction,
        }
    }
}
