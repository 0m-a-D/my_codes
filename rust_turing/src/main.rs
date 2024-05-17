mod direction;
mod state;
mod tape;
mod transition;
mod turing;

use crate::direction::Direction;
use crate::state::State;
use crate::tape::Tape;
use crate::transition::Transition;
use crate::turing::TuringMach;

fn main() {
    let input = "1100100"; // Binary string
    let tape = Tape::new(input);

    let transitions = vec![
        Transition::new(State::Start, '0', State::Start, '0', Direction::Right),
        Transition::new(State::Start, '1', State::Start, '1', Direction::Right),
        Transition::new(State::Start, ' ', State::Check, ' ', Direction::Left),
        Transition::new(State::Check, '0', State::Zero, '0', Direction::Left),
        Transition::new(State::Zero, '0', State::Accept, '0', Direction::Left),
        Transition::new(State::Zero, '1', State::Reject, '1', Direction::Right),
        Transition::new(State::Check, '1', State::Reject, '1', Direction::Right),
        Transition::new(State::Check, ' ', State::Reject, ' ', Direction::Right),
    ];

    let mut machine = TuringMach::new(tape, transitions);

    machine.run();

    if machine.is_accepted() {
        println!("Accepted: {}", input);
    } else {
        println!("Rejected: {}", input);
    }
}
