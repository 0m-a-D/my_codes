use crate::direction::Direction;

pub struct Tape {
    pub head_pos: i32,
    pub tape: Vec<char>,
}
#[allow(clippy::inherent_to_string)]
impl Tape {
    pub fn new(input: &str) -> Self {
        let mut tape: Vec<char> = input.chars().collect();
        tape.push(' ');
        Self { tape, head_pos: 0 }
    }
    pub fn read(&self) -> char {
        self.tape[self.head_pos as usize]
    }
    pub fn write(&mut self, symbol: char) {
        self.tape[self.head_pos as usize] = symbol;
    }
    pub fn move_head_ptr(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.head_pos > 0 {
                    self.head_pos -= 1;
                }
            }
            Direction::Right => {
                self.head_pos += 1;
                if self.head_pos as usize == self.tape.len() {
                    self.tape.push(' ');
                }
            }
        }
    }
}
