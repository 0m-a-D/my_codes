#[derive(Copy, PartialEq, Clone, Eq, Hash, Debug)]
pub enum State {
    Start,
    Check,
    Zero,
    Accept,
    Reject,
}
