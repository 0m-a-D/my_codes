#[derive(Copy)]
pub enum Direction {
    Right,
    Left,
}
impl Clone for Direction {
    fn clone(&self) -> Self {
        *self
    }
}
